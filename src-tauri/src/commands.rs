use tauri::AppHandle;

use crate::{
    player::PlayerState,
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
        .map_err(|err| AppError::Player("play".into(), err))?;

    Ok(name)
}

#[tauri::command]
pub async fn pause(state: tauri::State<'_, AppState>) -> Result<(), AppError> {
    state
        .player
        .pause()
        .await
        .map_err(|err| AppError::Player("pause".into(), err))
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
        .map_err(|err| AppError::Player("set_volume".into(), err))
}

#[tauri::command]
pub async fn get_volume(state: tauri::State<'_, AppState>) -> Result<f32, AppError> {
    state
        .player
        .get_volume()
        .await
        .map_err(|err| AppError::Player("get_volume".into(), err))
}

#[tauri::command]
pub async fn get_player_state(state: tauri::State<'_, AppState>) -> Result<PlayerState, AppError> {
    state
        .player
        .get_player_state()
        .await
        .map_err(|err| AppError::Player("get_player_state".into(), err))
}

#[tauri::command]
pub fn stations() -> Result<Vec<radios::Station>, AppError> {
    Ok(radios::get_stations())
}
