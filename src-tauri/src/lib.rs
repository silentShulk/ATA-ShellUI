mod data;
mod features;

mod bindings;
use bindings::{install_mod, launch_automata, list_mods, list_types};

use crate::data::mods::ModType;
use strum::IntoEnumIterator;

use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::new()
        .commands(collect_commands![
            list_types,
            install_mod,
            list_mods,
            launch_automata
        ])
        .constant("MOD_TYPES", ModType::iter().collect::<Vec<_>>());

    #[cfg(debug_assertions)]
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("failed to export bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running application");
}