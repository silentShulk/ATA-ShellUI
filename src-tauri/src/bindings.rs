use std::{error::Error, path::PathBuf, sync::RwLock};
use tauri::{AppHandle, Manager};

use ata::{
    data::{
        mods::{Mod, Mods}, settings::Settings
    }, features::{
        installation::install_mod, mod_managing::{list_mods, toggle_mod}, uninstallation::uninstall_mod
    }
};

async fn run_blocking<T, C>(app: AppHandle, command: C) -> T
where
    C: FnOnce(AppHandle) -> T + Send + 'static,
    T: Send + 'static,
{
    tauri::async_runtime::spawn_blocking(move || command(app))
        .await
        .expect("Backend Panicked")
}

async fn try_run_blocking<T, E, C>(app: AppHandle, command: C) -> Result<T, String>
where
    C: FnOnce(AppHandle) -> Result<T, E> + Send + 'static,
    T: Send + 'static,
    E: Error + Send + 'static,
{
    run_blocking(app, command)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn install_mod_command(
    mod_archive_path: PathBuf,
    answered_name: String,
    overwrite_flag: bool,
    app: AppHandle,
) -> Result<Mod, String> {
    try_run_blocking(app, move |app| {
        let settings = app.state::<RwLock<Settings>>();
        let settings = settings.read().unwrap();
        
        let mods = app.state::<RwLock<Mods>>();
        let mut mods = mods.write().unwrap();

        install_mod(&mod_archive_path, &answered_name, overwrite_flag, &settings, &mut mods)
    })
    .await
}

#[tauri::command]
#[specta::specta]
pub async fn uninstall_mod_command(
    mod_name: String,
    app: AppHandle
) -> Result<Mod, String> {
    try_run_blocking(app, move |app| {
        let mods = app.state::<RwLock<Mods>>();
        let mut mods = mods.write().unwrap();

        uninstall_mod(&mod_name, &mut mods)
    })
    .await
}

#[tauri::command]
#[specta::specta]
pub async fn toggle_mod_command(
    mod_name: String,
    app: AppHandle
) -> Result<Mod, String> {
    try_run_blocking(app, move |app| {
        let mods = app.state::<RwLock<Mods>>();
        let mut mods = mods.write().unwrap();

        toggle_mod(&mod_name, &mut mods)
    })
    .await
}

#[tauri::command]
#[specta::specta]
pub async fn list_mods_command(app: AppHandle) -> Vec<Mod> {
    run_blocking(app, move |app| {
        let settings = app.state::<RwLock<Settings>>();
        let mods = app.state::<RwLock<Mods>>();
        
        let settings = settings.read().unwrap();
        let mods = mods.read().unwrap();
        
        list_mods(&settings.sorting_order, &mods.mods)
    })
    .await
}

#[tauri::command]
#[specta::specta]
pub async fn update_setting_command(
    setting: String,
    value: String,
    app: AppHandle
) -> Result<Settings, String> {
    try_run_blocking(app, move |app| {
        let settings = app.state::<RwLock<Settings>>();
        let mut settings = settings.write().unwrap();
        
        settings.update_setting(&setting, &value)
    })
    .await
}

#[tauri::command]
#[specta::specta]
pub async fn list_settings_command(app: AppHandle) -> Vec<Mod> {
    run_blocking(app, move |app| {
        let settings = app.state::<RwLock<Settings>>();
        let mods = app.state::<RwLock<Mods>>();
        
        let settings = settings.read().unwrap();
        let mods = mods.read().unwrap();
        
        list_mods(&settings.sorting_order, &mods.mods)
    })
    .await
}

#[tauri::command]
#[specta::specta]
pub fn launch_automata_command() -> Result<(), String> {
    tauri_plugin_opener::open_url("steam://rungameid/524220", None::<&str>).map_err(|err| err.to_string())
}