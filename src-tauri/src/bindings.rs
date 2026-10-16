use crate::features::misc::launch_automata_inner;

#[tauri::command]
pub fn launch_automata() -> Result<(), String> {
    launch_automata_inner().map_err(|err| err.to_string())
}