use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            if let Ok(Some(monitor)) = window.current_monitor() {
                let screen_size = monitor.size();
                let scale_factor = monitor.scale_factor();

                let window_size = window.outer_size()?;
                let logical_width = (window_size.width as f64 / scale_factor) as u32;
                let logical_height = (window_size.height as f64 / scale_factor) as u32;

                let x = ((screen_size.width - logical_width) / 2) as i32;
                let y = (screen_size.height - logical_height - 100) as i32;

                window.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(
                    x, y,
                )))?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
