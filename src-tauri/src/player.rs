use std::error::Error;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex, MutexGuard};

use icy_metadata::{IcyHeaders, IcyMetadataReader, RequestIcyMetadata};
use rodio::{OutputStream, OutputStreamBuilder, Sink, StreamError};
use stream_download::http::reqwest::Client;
use stream_download::http::HttpStream;
use stream_download::source::DecodeError;
use stream_download::storage::bounded::BoundedStorageProvider;
use stream_download::storage::memory::MemoryStorageProvider;
use stream_download::{Settings, StreamDownload};
use tauri::{AppHandle, Emitter};

use crate::radios::Station;

pub struct Player {
    sink: Arc<Mutex<Sink>>,
    _stream: OutputStream,
}

#[derive(Debug)]
pub enum PlayerError {
    StreamCreationError(StreamError),
    SinkLockError(),
}

// buffer 5 seconds of audio
// bitrate (in kilobits) / bits per byte * bytes per kilobyte * 5 seconds
fn get_prefetch_bytes(bitrate: Option<u32>) -> u64 {
    bitrate
        .map(|v| (v / 8 * 1024 * 5) as u64)
        .unwrap_or_else(|| (256 * 1024) as u64)
}

impl Player {
    pub fn new() -> Result<Self, PlayerError> {
        let _stream = match OutputStreamBuilder::open_default_stream() {
            Ok(s) => s,
            Err(e) => {
                return Err(PlayerError::StreamCreationError(e));
            }
        };
        let sink = rodio::Sink::connect_new(_stream.mixer());
        sink.set_volume(0.2);

        Ok(Self {
            sink: Arc::new(Mutex::new(sink)),
            _stream,
        })
    }

    pub async fn play(
        &self,
        app: AppHandle,
        station: &Station,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        // We need to add a header to tell the Icecast server that we can parse the metadata embedded
        // within the stream itself.
        let client = Client::builder().request_icy_metadata().build()?;

        let stream = HttpStream::new(client, station.get_url().parse()?).await?;

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
            Err(e) => Err(e.decode_error().await)?,
        };

        // Appending the stream to the sink has to be done in a separate thread, otherwise no sound will play
        let sink = Arc::clone(&self.sink);
        let metadata_interval = icy_headers.metadata_interval();
        let handle = tauri::async_runtime::spawn_blocking(move || {
            let sink = sink.lock().unwrap();
            sink.stop(); // Stop the current stream, if any
            sink.append(rodio::Decoder::new(IcyMetadataReader::new(
                reader,
                metadata_interval, // If interval is present, fetch new data after interval has passed
                // Emit the stream metadata whenever we receive new values
                move |metadata| {
                    app.emit("title", metadata.unwrap().stream_title().unwrap_or(""))
                        .unwrap()
                },
            ))?);
            Ok::<_, Box<dyn Error + Send + Sync>>(())
        });
        handle.await??;
        Ok(station.get_name().to_string())
    }

    pub fn pause(&self) -> Result<(), PlayerError> {
        let sink = self.get_sink()?;

        sink.stop();
        Ok(())
    }

    pub fn get_volume(&self) -> Result<f32, PlayerError> {
        self.get_sink()
            .map(|sink| sink.volume() * 100.0)
            .map(f32::round)
    }

    pub fn set_volume(&self, app: AppHandle, volume: f32) -> Result<(), PlayerError> {
        let sink = self.get_sink()?;
        sink.set_volume(volume.clamp(0.0, 100.0) / 100.0);
        drop(sink);

        app.emit("volume_change", self.get_volume()?).unwrap();
        Ok(())
    }

    fn get_sink(&self) -> Result<MutexGuard<'_, Sink>, PlayerError> {
        self.sink.lock().map_err(|_| PlayerError::SinkLockError())
    }
}
