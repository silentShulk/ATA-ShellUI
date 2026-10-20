use crate::features::installation::install_mod_inner;
use crate::features::misc::launch_automata_inner;

#[tauri::command]
pub fn launch_automata() -> Result<(), String> {
    launch_automata_inner().map_err(|err| err.to_string())
}

#[tauri::command]
pub fn install_mod(app: tauri::AppHandle, answered_name: &str) {
    install_mod_inner(app, answered_name).map_err(|err| err.to_string())
}