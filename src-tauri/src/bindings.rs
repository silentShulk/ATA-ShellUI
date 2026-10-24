use crate::features::installation::install_mod_inner;
use crate::features::misc::launch_automata_inner;

#[tauri::command]
pub fn launch_automata() -> Result<(), String> {
    launch_automata_inner().map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn install_mod(
    app: tauri::AppHandle,
    answered_name: String,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || install_mod_inner(app, &answered_name))
        .await
        .map_err(|err| err.to_string())?
        .map_err(|err| err.to_string())
}