use std::{path::PathBuf, sync::Mutex};
use tauri::{AppHandle, State, Emitter};

use ata::{
    features::{
        installation::install_mod,
        mod_managing::list_mods
    },
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
    mods: State<Mutex<Mods>>,
    app: AppHandle
) -> Result<Mod, String> {
    let settings = settings.lock().unwrap();
    let mut mods = mods.lock().unwrap();

    install_mod(
        &compressed_mod_folder_path,
        &answered_name,
        forced_overwrite,
        &settings,
        &mut mods,
    ).map_err(|e| {
        app.emit("error", e.to_string()).unwrap();
        e.to_string()
    })
}

#[tauri::command]
#[specta::specta]
pub fn list_mods_command(mods: State<Mods>, settings: State<Settings>) -> Vec<Mod> {
    list_mods(&settings.sorting_order, &mods.mods)
}

#[tauri::command]
#[specta::specta]
pub fn launch_automata_command() -> Result<(), String> {
    tauri_plugin_opener::open_url("steam://rungameid/524220", None::<&str>).map_err(|err| err.to_string())
}