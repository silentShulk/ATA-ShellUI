use crate::settings::Settings;
use std::path::PathBuf;

use std::sync::Mutex;
use tauri::State;

use crate::settings::{Style, Palette, SortingOrder, ConflictResolution};



fn set_style_inner(settings: &mut Settings, style: Style) -> Result<Settings, crate::settings::SettingsInteractionError> {
    settings.style = style;
    settings.update_settings_file()?;

    Ok(settings.clone())
}
fn set_palette_inner(settings: &mut Settings, palette: Palette) -> Result<Settings, crate::settings::SettingsInteractionError> {
    settings.palette = palette;
    settings.update_settings_file()?;

    Ok(settings.clone())
}
fn set_sorting_order_inner(settings: &mut Settings, sorting_order: SortingOrder) -> Result<Settings, crate::settings::SettingsInteractionError> {
    settings.sorting_order = sorting_order;
    settings.update_settings_file()?;

    Ok(settings.clone())
}
fn set_files_conflict_resolution_inner(settings: &mut Settings, conflict_resolution: ConflictResolution) -> Result<Settings, crate::settings::SettingsInteractionError> {
    settings.files_conflict_resolution = conflict_resolution;
    settings.update_settings_file()?;

    Ok(settings.clone())
}
fn set_keep_extracted_folders_inner(settings: &mut Settings, keep_extracted_folders: bool) -> Result<Settings, crate::settings::SettingsInteractionError> {
    settings.keep_extracted_folders = keep_extracted_folders;
    settings.update_settings_file()?;

    Ok(settings.clone())
}
fn set_extracted_folders_location_inner(settings: &mut Settings, extracted_folders_location: PathBuf) -> Result<Settings, crate::settings::SettingsInteractionError> {
    settings.extracted_folders_location = extracted_folders_location;
    settings.update_settings_file()?;

    Ok(settings.clone())
}
fn set_game_path_inner(settings: &mut Settings, game_path: PathBuf) -> Result<Settings, crate::settings::SettingsInteractionError> {
    settings.game_path = game_path;
    settings.update_settings_file()?;

    Ok(settings.clone())
}
fn set_discord_rich_presence_inner(settings: &mut Settings, discord_rich_presence: String) -> Result<Settings, crate::settings::SettingsInteractionError> {
    settings.discord_rich_presence = discord_rich_presence;
    settings.update_settings_file()?;

    Ok(settings.clone())
}

#[tauri::command]
pub fn set_style(settings: State<Mutex<Settings>>, style: Style) -> Result<Settings, String> {
    let mut settings = settings.lock().unwrap();
    
    set_style_inner(&mut settings, style).map_err(|e| e.to_string())
}
#[tauri::command]
pub fn set_palette(settings: State<Mutex<Settings>>, palette: Palette) -> Result<Settings, String> {
    let mut settings = settings.lock().unwrap();
    
    set_palette_inner(&mut settings, palette).map_err(|e| e.to_string())
}
#[tauri::command]
pub fn set_sorting_order(settings: State<Mutex<Settings>>, sorting_order: SortingOrder) -> Result<Settings, String> {
    let mut settings = settings.lock().unwrap();
    
    set_sorting_order_inner(&mut settings, sorting_order).map_err(|e| e.to_string())
}
#[tauri::command]
pub fn set_files_conflict_resolution(settings: State<Mutex<Settings>>, files_conflict_resolution: ConflictResolution) -> Result<Settings, String> {
    let mut settings = settings.lock().unwrap();
    
    set_files_conflict_resolution_inner(&mut settings, files_conflict_resolution).map_err(|e| e.to_string())
}
#[tauri::command]
pub fn set_keep_extracted_folders(settings: State<Mutex<Settings>>, keep_extracted_folders: bool) -> Result<Settings, String> {
    let mut settings = settings.lock().unwrap();
    
    set_keep_extracted_folders_inner(&mut settings, keep_extracted_folders).map_err(|e| e.to_string())
}
#[tauri::command]
pub fn set_extracted_folders_location(settings: State<Mutex<Settings>>, extracted_folders_location: PathBuf) -> Result<Settings, String> {
    let mut settings = settings.lock().unwrap();
    
    set_extracted_folders_location_inner(&mut settings, extracted_folders_location).map_err(|e| e.to_string())
}
#[tauri::command]
pub fn set_game_path(settings: State<Mutex<Settings>>, game_path: PathBuf) -> Result<Settings, String> {
    let mut settings = settings.lock().unwrap();
    
    set_game_path_inner(&mut settings, game_path).map_err(|e| e.to_string())
}
#[tauri::command]
pub fn set_discord_rich_presence(settings: State<Mutex<Settings>>, discord_rich_presence: String) -> Result<Settings, String> {
    let mut settings = settings.lock().unwrap();
    
    set_discord_rich_presence_inner(&mut settings, discord_rich_presence).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_settings(settings: State<Mutex<Settings>>) -> Result<Settings, String> {
    let settings = settings.lock().unwrap();
    Ok(settings.clone())
}
