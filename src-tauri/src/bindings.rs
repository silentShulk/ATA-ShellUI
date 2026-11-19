use std::{path::PathBuf, sync::Mutex};
use tauri::{AppHandle, State, Emitter};

use ata::{
    features::installation::install_mod,
    data::{
        mods::{Mods, Mod},
        settings::Settings
    }
};

#[tauri::command]
#[specta::specta]
pub fn install_mod_command(
    compressed_mod_folder_path: PathBuf,
    answered_name: String,
    forced_overwrite: bool,
    settings: State<Mutex<Settings>>,
    data: State<Mutex<Mods>>,
    app: AppHandle
) -> Result<Mod, String> {
    let settings = settings.lock().unwrap();
    let mut data = data.lock().unwrap();

    install_mod(
        &compressed_mod_folder_path,
        &answered_name,
        forced_overwrite,
        &settings,
        &mut data,
    ).map_err(|e| {
        app.emit("error", e.to_string()).unwrap();
        e.to_string()
    })
}

#[tauri::command]
#[specta::specta]
pub fn list_mods_command() -> Vec<Mod> {
    Vec::new().into()
}

#[tauri::command]
#[specta::specta]
pub fn launch_automata_command() -> Result<(), String> {
    tauri_plugin_opener::open_url("steam://rungameid/524220", None::<&str>).map_err(|err| err.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn load_data_command() -> Result<Mods, String> {
    Mods::load_data().map_err(|err| err.to_string())
}