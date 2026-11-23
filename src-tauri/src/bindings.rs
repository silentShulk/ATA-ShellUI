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

async fn dispatch<T, C>(command: C) -> Result<T, String>
where
    C: FnOnce() -> Result<T, String> + Send + 'static,
    T: Send + 'static,
{
    tauri::async_runtime::spawn_blocking(command)
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
#[specta::specta]
pub async fn install_mod_command(
    mod_archive_path: PathBuf,
    answered_name: String,
    overwrite_flag: bool,
    settings: State<'_, Settings>,
    mods: State<'_, Mutex<Mods>>,
    app: AppHandle
) -> Result<Mod, String> {
    let mut mods = mods.lock().unwrap();

    dispatch(|| install_mod(&mod_archive_path, &answered_name, overwrite_flag, &settings, &mut mods))
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