use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    App, Manager, Wry,
};

pub fn build_tray(app: &mut App) -> Result<TrayIcon<Wry>, tauri::Error> {
    let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let hide_i = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show_i, &hide_i])?;

    TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                println!("show menu item was clicked");
                if let Some(window) = app.get_webview_window("main") {
                    window.show().unwrap_or(());
                }
            }
            "hide" => {
                println!("hide menu item was clicked");
                if let Some(window) = app.get_webview_window("main") {
                    window.hide().unwrap_or(());
                }
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .build(app)
}
