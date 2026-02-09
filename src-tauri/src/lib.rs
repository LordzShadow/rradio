mod commands;
mod player;
mod radios;
mod tray;

pub use crate::player::Player;

struct AppState {
    player: Player,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let player = Player::new().unwrap();
    let state = AppState { player };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .setup(|app| {
            tray::build_tray(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::play,
            commands::pause,
            commands::stations,
            commands::set_volume,
            commands::get_volume
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
