use std::sync::Mutex;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod data;
mod installation;
mod uninstallation;
mod mod_managing;
mod settings;
mod settings_managing;

use data::Data;
use installation::install_mod;
use uninstallation::uninstall_mod;
use mod_managing::list_mods;
use mod_managing::enable_mod;
use mod_managing::disable_mod;
use settings::Settings;
use settings_managing::{
    get_settings,
    set_style, set_palette, set_sorting_order,
    set_files_conflict_resolution, set_keep_extracted_folders, set_extracted_folders_location,
    set_game_path, set_discord_rich_presence
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(Data::load_data().unwrap()))
        .manage(Mutex::new(Settings::load_settings().unwrap()))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            install_mod,
            uninstall_mod,
            list_mods,
            enable_mod,
            disable_mod,
            get_settings,
            
            set_style,
            set_palette,
            set_sorting_order,
            set_files_conflict_resolution,
            set_keep_extracted_folders,
            set_extracted_folders_location,
            set_game_path,
            set_discord_rich_presence,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
