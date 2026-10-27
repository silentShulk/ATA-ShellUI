mod data;
mod features;

mod bindings;
use bindings::{install_mod, list_mods, launch_automata};
mod data;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            install_mod,
            list_mods,
            launch_automata
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
