#[tauri::command]
pub fn launch_automata() -> Result<(), String> {
    launch_automata_inner().map_err(|err| err.to_string())
}
fn launch_automata_inner() -> Result<(), tauri_plugin_opener::Error> {
    tauri_plugin_opener::open_url("steam://rungameid/524220", None::<&str>)
}