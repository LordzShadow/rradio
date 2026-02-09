mod player;
mod radios;
mod tray;

use tauri::AppHandle;

pub use crate::player::Player;
use crate::{
    radios::{get_station_by_uuid, get_stations},
    tray::build_tray,
};

struct AppState {
    player: Player,
}

#[tauri::command]
async fn play(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    uuid: &str,
) -> Result<String, String> {
    let station = get_station_by_uuid(uuid).ok_or("Station not found")?;
    let name = state
        .player
        .play(app, &station)
        .await
        .map_err(|err| err.to_string())?;

    Ok(name)
}

#[tauri::command]
async fn pause(state: tauri::State<'_, AppState>) -> Result<(), String> {
    state.player.pause();
    Ok(())
}

#[tauri::command]
fn stations() -> Vec<radios::Station> {
    get_stations()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let player = Player::new().unwrap();
    let state = AppState { player };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .setup(|app| {
            build_tray(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![play, pause, stations])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
