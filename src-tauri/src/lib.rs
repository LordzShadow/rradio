mod player;
mod radios;

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};

pub use crate::player::Player;
use crate::radios::get_stations;

struct AppState {
    player: Player,
}

#[tauri::command]
async fn play(app: AppHandle, url: &str) -> Result<String, String> {
    let name = app
        .state::<AppState>()
        .player
        .play(url)
        .await
        .map_err(|err| err.to_string())?;

    Ok(format!("Playing {}!", name))
}

#[tauri::command]
async fn pause(state: tauri::State<'_, AppState>) -> Result<(), String> {
    state.player.pause().map_err(|err| err.to_string())?;
    Ok(())
}

#[tauri::command]
fn stations() -> Vec<radios::Station> {
    get_stations()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let player = Player::new().unwrap();
    let state = AppState { player: player };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .setup(|app| {
            let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let hide_i = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&show_i, &hide_i])?;

            TrayIconBuilder::new()
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        println!("show menu item was clicked");
                        app.get_webview_window("main").unwrap().show();
                    }
                    "hide" => {
                        println!("hide menu item was clicked");
                        app.get_webview_window("main").unwrap().hide();
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![play, pause, stations])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
