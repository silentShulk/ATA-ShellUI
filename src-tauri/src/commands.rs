use tauri::{AppHandle, Emitter, State};
use std::sync::Mutex;

use std::path::PathBuf;

use crate::data::{Data, Mod};
use crate::settings::{
    Settings,
};
use crate::installation::install_mod;
use crate::uninstallation::uninstall_mod;
use crate::mod_managing::{
    list_mods,
    enable_mod,
    disable_mod,
};




#[tauri::command]
pub fn install_mod_command(
    compressed_mod_folder_path: PathBuf,
    answered_name: String,
    forced_overwrite: bool,
    settings: State<Mutex<Settings>>,
    data: State<Mutex<Data>>,
    app: AppHandle
) -> Result<Mod, String> {
    let settings = settings.lock().unwrap();
    let mut data = data.lock().unwrap();

    install_mod(
        &compressed_mod_folder_path,
        answered_name,
        forced_overwrite,
        &settings,
        &mut data,
        &app
    ).map_err(|e| {
        app.emit("error", e.to_string()).unwrap();
        e.to_string()
    })
}

#[tauri::command]
pub fn uninstall_mod_command(
    mod_name: String,
    config: State<Mutex<Data>>,
) -> Result<Mod, String> {
    let mut config = config.lock().unwrap();
    
    uninstall_mod(
        &mut config,
        mod_name,
    ).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_mods_command(
    settings: State<Mutex<Settings>>,
    data: State<Mutex<Data>>
) -> Vec<Mod> {
    let settings = settings.lock().unwrap();
    let data = data.lock().unwrap();
    
	list_mods(&settings.sorting_order, &data.mods)
}

#[tauri::command]
pub fn enable_mod_command(
    mod_name: String,
    data: State<Mutex<Data>>,
) -> Result<Mod, String> {
    let mut data = data.lock().unwrap();
    
    enable_mod(&mut data, mod_name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn disable_mod_command(
    mod_name: String,
    data: State<Mutex<Data>>,
) -> Result<Mod, String> {
    let mut data = data.lock().unwrap();
    
    disable_mod(&mut data, mod_name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_settings_command(
    settings: State<Mutex<Settings>>,
) -> Settings {
    settings.lock().unwrap().clone()
}

#[tauri::command]
pub fn update_setting_command(
    setting: String,
    value: String,
    settings: State<Mutex<Settings>>,
) -> Result<Settings, String> {
    let mut settings = settings.lock().unwrap();
    
    settings.update_setting(setting, value)
        .map_err(|e| e.to_string())
}
