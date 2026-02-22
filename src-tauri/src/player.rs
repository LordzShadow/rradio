use std::num::NonZeroUsize;
use std::sync::Arc;

use icy_metadata::{IcyHeaders, IcyMetadataReader, RequestIcyMetadata};
use rodio::{DeviceSinkBuilder, DeviceSinkError, MixerDeviceSink};
use stream_download::http::reqwest::Client;
use stream_download::http::HttpStream;
use stream_download::source::DecodeError;
use stream_download::storage::bounded::BoundedStorageProvider;
use stream_download::storage::memory::MemoryStorageProvider;
use stream_download::{Settings, StreamDownload};
use tauri::async_runtime::{Mutex, RwLock};
use tauri::{AppHandle, Emitter};

use crate::radios::Station;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerState {
    current_station_uuid: Option<String>,
    volume: f32,
    playing: bool,
    track_title: Option<String>,
}

pub struct Player {
    audio_player: Arc<Mutex<rodio::Player>>,
    _sink: MixerDeviceSink,
    current_station_uuid: Arc<RwLock<Option<String>>>,
    track_title: Arc<RwLock<Option<String>>>,
}

#[derive(Debug, thiserror::Error)]
pub enum PlayerError {
    #[error("Failed to create sink")]
    SinkCreation(#[from] DeviceSinkError),
    #[error("Failed to emit event")]
    AppEmit(#[from] tauri::Error),
    #[error("Failed to play stream")]
    StreamPlayback(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("Failed to download stream")]
    StreamDownload(Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("Failed to initialize stream: {0}")]
    StreamInit(String),
}

// buffer 5 seconds of audio
// bitrate (in kilobits) / bits per byte * bytes per kilobyte * 5 seconds
fn get_prefetch_bytes(bitrate: Option<u32>) -> u64 {
    bitrate
        .map(|v| (v / 8 * 1024 * 5) as u64)
        .unwrap_or_else(|| (256 * 1024) as u64)
}

fn player_volume_to_percent(volume: f32) -> f32 {
    (volume * 100.0).round()
}

fn percent_volume_to_player(volume: f32) -> f32 {
    volume / 100.0
}

impl Player {
    pub fn new() -> Result<Self, PlayerError> {
        let _stream = match DeviceSinkBuilder::open_default_sink() {
            Ok(s) => s,
            Err(e) => {
                return Err(PlayerError::SinkCreation(e));
            }
        };
        let player = rodio::Player::connect_new(_stream.mixer());
        player.set_volume(0.2);

        Ok(Self {
            audio_player: Arc::new(Mutex::new(player)),
            _sink: _stream,
            current_station_uuid: Arc::new(RwLock::new(None)),
            track_title: Arc::new(RwLock::new(None)),
        })
    }

    pub async fn play(&self, app: AppHandle, station: &Station) -> Result<String, PlayerError> {
        // We need to add a header to tell the Icecast server that we can parse the metadata embedded
        // within the stream itself.
        let client = Client::builder()
            .request_icy_metadata()
            .build()
            .map_err(|err| PlayerError::StreamPlayback(Box::new(err)))?;

        let stream = HttpStream::new(
            client,
            station
                .get_url()
                .parse()
                .map_err(|err| PlayerError::StreamDownload(Box::new(err)))?,
        )
        .await
        .map_err(|err| PlayerError::StreamDownload(Box::new(err)))?;

        let icy_headers = IcyHeaders::parse_from_headers(stream.headers());

        let prefetch_bytes = get_prefetch_bytes(icy_headers.bitrate());

        let reader = match StreamDownload::from_stream(
            stream,
            // use bounded storage to keep the underlying size from growing indefinitely
            BoundedStorageProvider::new(
                MemoryStorageProvider,
                // be liberal with the buffer size, you need to make sure it holds enough space to
                // prevent any out-of-bounds reads
                NonZeroUsize::new(512 * 1024).unwrap(),
            ),
            Settings::default().prefetch_bytes(prefetch_bytes),
        )
        .await
        {
            Ok(reader) => reader,
            Err(err) => Err(PlayerError::StreamInit(err.decode_error().await))?,
        };

        // Appending the stream to the player has to be done in a separate thread, otherwise no sound will play
        let audio_player = Arc::clone(&self.audio_player);
        let track_title = Arc::clone(&self.track_title);
        let metadata_interval = icy_headers.metadata_interval();
        let handle = tauri::async_runtime::spawn(async move {
            // Reset title on station change
            {
                let mut title = track_title.write().await;
                *title = None;
            }

            let audio_player = audio_player.lock().await;
            audio_player.stop(); // Stop the current stream, if any
            audio_player.append(
                rodio::Decoder::new(IcyMetadataReader::new(
                    reader,
                    metadata_interval, // If interval is present, fetch new data after interval has passed
                    // Emit the stream title whenever we receive new values
                    move |metadata| {
                        let stream_title = metadata
                            .map(|meta| meta.stream_title().map(|title| title.to_string()))
                            .ok()
                            .flatten();
                        let mut title = track_title.blocking_write();
                        *title = stream_title.clone();
                        drop(title);
                        app.emit("title", stream_title).unwrap_or(())
                    },
                ))
                .map_err(|err| PlayerError::StreamPlayback(Box::new(err)))?,
            );
            Ok::<_, PlayerError>(())
        });
        handle
            .await
            .map_err(|err| PlayerError::StreamPlayback(Box::new(err)))??;

        let mut current_station_uuid = self.current_station_uuid.write().await;
        *current_station_uuid = Some(station.get_uuid().to_string());

        Ok(station.get_name().to_string())
    }

    pub async fn pause(&self) -> Result<(), PlayerError> {
        let audio_player = self.audio_player.lock().await;
        let mut title = self.track_title.write().await;
        *title = None;

        audio_player.stop();
        Ok(())
    }

    pub async fn get_volume(&self) -> Result<f32, PlayerError> {
        let audio_player = self.audio_player.lock().await;
        Ok(player_volume_to_percent(audio_player.volume()))
    }

    pub async fn set_volume(&self, app: AppHandle, volume: f32) -> Result<(), PlayerError> {
        let audio_player = self.audio_player.lock().await;
        audio_player.set_volume(percent_volume_to_player(volume).clamp(0.0, 1.0));

        app.emit(
            "volume_change",
            player_volume_to_percent(audio_player.volume()),
        )
        .map_err(PlayerError::AppEmit)?;
        Ok(())
    }

    pub async fn get_player_state(&self) -> Result<PlayerState, PlayerError> {
        let (playing, volume) = {
            let audio_player = self.audio_player.lock().await;
            let playing = !audio_player.empty(); // stop does not mark player as paused, so we use queue size here instead
            let volume = player_volume_to_percent(audio_player.volume());
            (playing, volume)
        };
        let current_station_uuid = self.current_station_uuid.read().await;
        let track_title = self.track_title.read().await;
        Ok(PlayerState {
            playing,
            volume,
            current_station_uuid: current_station_uuid.clone(),
            track_title: track_title.clone(),
        })
    }
}
