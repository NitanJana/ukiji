use rdev::{listen, Event, EventType};
use std::thread;
use tauri::{Emitter, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            // click-through window
            window.set_ignore_cursor_events(true)?;

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

                window.show()?;
            }

            let app_handle = app.handle().clone();

            thread::spawn(move || {
                let callback = move |event: Event| {
                    if let EventType::KeyPress(key) = event.event_type {
                        let payload = event.name.unwrap_or_else(|| format!("{:?}", key));
                        println!("Global Key Press: {}", payload);

                        let _ = app_handle.emit("GLOBAL_KEY", payload);
                    }
                };

                if let Err(error) = listen(callback) {
                    eprintln!("Error in global listener: {:?}", error);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
