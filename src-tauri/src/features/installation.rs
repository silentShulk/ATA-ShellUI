use std::time::Duration;

use tauri::{AppHandle, Emitter};

pub fn install_mod_inner(app: AppHandle, answered_name: &str) -> Result<(), tauri::Error> {
    app.emit("setup", "Performing checks on path and name")?;
    std::thread::sleep(Duration::from_secs(1));

    app.emit("decompression", "Decompressing mod archive")?;
    std::thread::sleep(Duration::from_secs(2));

    app.emit("mod-analysis", "Analising mod files")?;
    std::thread::sleep(Duration::from_secs(1));

    app.emit(
        "conflicts",
        "Checking for conflicts with already installed mods",
    )?;
    std::thread::sleep(Duration::from_secs(1));

    app.emit("installation", "Installing the mod")?;
    std::thread::sleep(Duration::from_secs(2));

    app.emit("data-update", "Updating the data file")?;
    std::thread::sleep(Duration::from_secs(1));

    app.emit(
        "success",
        format!("\n{}\n installed successfully", answered_name),
    )?;
    std::thread::sleep(Duration::from_secs(1));

    Ok(())
}
