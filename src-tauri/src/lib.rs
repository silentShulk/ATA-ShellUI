mod bindings;
use bindings::{install_mod_command, launch_automata_command, list_mods_command};

use ata::data::{
    mods::{ModType, Mods},
    settings::Settings
};

use strum::IntoEnumIterator;
use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::new()
        .commands(collect_commands![
            install_mod_command,
            list_mods_command,
            launch_automata_command
        ])
        .constant("MOD_TYPES", ModType::iter().collect::<Vec<_>>());

    #[cfg(debug_assertions)]
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("failed to export bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(builder.invoke_handler())
        .manage(Mods::load_data())
        .manage(Settings::load_settings())
        .setup(move |app| {
            builder.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running application");
}