// mod data;
mod features;
// mod utils;

mod bindings;
use bindings::{
    install_mod,
    launch_automata
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            install_mod,
            launch_automata
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
