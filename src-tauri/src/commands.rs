use tauri::AppHandle;

use crate::{
    radios::{self},
    AppState,
};

#[tauri::command]
pub async fn play(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    uuid: &str,
) -> Result<String, String> {
    let station = radios::get_station_by_uuid(uuid).ok_or("Station not found")?;
    let name = state
        .player
        .play(app, &station)
        .await
        .map_err(|err| err.to_string())?;

    Ok(name)
}

#[tauri::command]
pub async fn pause(state: tauri::State<'_, AppState>) -> Result<(), String> {
    state
        .player
        .pause()
        .await
        .map_err(|_| "Failed to pause".into())
}

#[tauri::command]
pub async fn set_volume(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    volume: f32,
) -> Result<(), String> {
    state
        .player
        .set_volume(app, volume)
        .await
        .map_err(|_| "Failed to set volume".into())
}

#[tauri::command]
pub async fn get_volume(state: tauri::State<'_, AppState>) -> Result<f32, String> {
    state
        .player
        .get_volume()
        .await
        .map_err(|_| "Failed to get volume".into())
}

#[tauri::command]
pub fn stations() -> Vec<radios::Station> {
    radios::get_stations()
}
