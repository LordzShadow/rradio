use tauri::AppHandle;

use crate::{
    radios::{self},
    AppError, AppState,
};

#[tauri::command]
pub async fn play(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    uuid: &str,
) -> Result<String, AppError> {
    let station =
        radios::get_station_by_uuid(uuid).ok_or(AppError::StationNotFound(uuid.to_string()))?;
    let name = state
        .player
        .play(app, &station)
        .await
        .map_err(|_| AppError::Command("play".into()))?;

    Ok(name)
}

#[tauri::command]
pub async fn pause(state: tauri::State<'_, AppState>) -> Result<(), AppError> {
    state
        .player
        .pause()
        .await
        .map_err(|_| AppError::Command("pause".into()))
}

#[tauri::command]
pub async fn set_volume(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    volume: f32,
) -> Result<(), AppError> {
    state
        .player
        .set_volume(app, volume)
        .await
        .map_err(|_| AppError::Command("set_volume".into()))
}

#[tauri::command]
pub async fn get_volume(state: tauri::State<'_, AppState>) -> Result<f32, AppError> {
    state
        .player
        .get_volume()
        .await
        .map_err(|_| AppError::Command("get_volume".into()))
}

#[tauri::command]
pub fn stations() -> Result<Vec<radios::Station>, AppError> {
    Ok(radios::get_stations())
}
