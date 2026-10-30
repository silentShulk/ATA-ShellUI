use std::path::PathBuf;

use chrono::Utc;

use crate::features::installation::install_mod_inner;
use crate::features::manage::list_mods_inner;
use crate::features::misc::launch_automata_inner;

use crate::data::mods::{Mod, ModType};

#[tauri::command]
#[specta::specta]
pub async fn install_mod(app: tauri::AppHandle, answered_name: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || install_mod_inner(app, &answered_name))
        .await
        .map_err(|err| err.to_string())?
        .map_err(|err| err.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn list_mods() -> Vec<Mod> {
    list_mods_inner(&[
        Mod::new(String::from("Giacomo"),
        Vec::from([
            PathBuf::from("~/Downloads/kirby/teto.cs"),
            PathBuf::from("~/Documents/kirby/dave.json"),
        ]),
        true,
        ModType::WorldModels,
        Utc::now(),
        ),
        Mod::new(String::from("Giacomo"),
        Vec::from([
            PathBuf::from("~/Downloads/kirby/teto.cs"),
            PathBuf::from("~/Documents/kirby/dave.json"),
        ]),
        false,
        ModType::WorldModels,
        Utc::now(),
        )
    ])
}

#[tauri::command]
#[specta::specta]
pub fn launch_automata() -> Result<(), String> {
    launch_automata_inner().map_err(|err| err.to_string())
}
