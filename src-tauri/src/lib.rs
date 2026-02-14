mod commands;
mod player;
mod radios;
mod tray;

pub use crate::player::Player;

#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("Failed to execute command {0}")]
    Command(String),
    #[error("Station {0} not found")]
    StationNotFound(String),
}

#[derive(serde::Serialize)]
#[serde(tag = "kind", content = "message")]
#[serde(rename_all = "camelCase")]
enum ErrorKind {
    Command(String),
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let error_message = self.to_string();
        let error_kind = match self {
            Self::Command(_) => ErrorKind::Command(error_message),
            Self::StationNotFound(_) => ErrorKind::Command(error_message),
        };
        error_kind.serialize(serializer)
    }
}

struct AppState {
    player: Player,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let player = Player::new().expect("Could not initialize player!");
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
        .expect("Error while running tauri application");
}
