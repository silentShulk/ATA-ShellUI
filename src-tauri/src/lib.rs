use std::sync::Mutex;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod backend;
use backend::{
    paths,
    data,
    installation,
    mod_managing,
    settings,
    uninstallation
};

use data::Data;
use settings::Settings;

mod commands;
use commands::{
    install_mod_command,
    uninstall_mod_command,
    list_mods_command,
    enable_mod_command,
    disable_mod_command,
    load_settings_command,
    update_setting_command
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(Data::load_data().unwrap()))
        .manage(Mutex::new(Settings::load_settings().unwrap()))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            install_mod_command,
            uninstall_mod_command,
            list_mods_command,
            enable_mod_command,
            disable_mod_command,
            load_settings_command,
            update_setting_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
