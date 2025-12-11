use rdev::{listen, Event, EventType, Key};
use std::thread;
use tauri::{AppHandle, Emitter};

/// spawns the background thread to listen for global keys
pub fn spawn_listener(app_handle: AppHandle) {
    thread::spawn(move || {
        let callback = move |event: Event| {
            if let EventType::KeyPress(key) = event.event_type {
                let label = get_key_label(key, event.name);
                println!("Key: {}", label);

                let _ = app_handle.emit("GLOBAL_KEY", label);
            }
        };

        if let Err(error) = listen(callback) {
            eprintln!("Error in global listener: {:?}", error);
        }
    });
}

fn get_key_label(key: Key, name: Option<String>) -> String {
    match key {
        // --- Functional ---
        Key::Space => "Space".to_string(),
        Key::Return => "Enter".to_string(),
        Key::Backspace => "Backspace".to_string(),
        Key::Tab => "Tab".to_string(),
        Key::Escape => "Esc".to_string(),

        // --- Modifiers ---
        Key::ShiftLeft | Key::ShiftRight => "Shift".to_string(),
        Key::ControlLeft | Key::ControlRight => "Ctrl".to_string(),
        Key::Alt | Key::AltGr => "Alt".to_string(),
        Key::MetaLeft | Key::MetaRight => "Win/Cmd".to_string(),
        Key::Function => "Fn".to_string(),

        _ => name.unwrap_or_else(|| format!("{:?}", key)),
    }
}
