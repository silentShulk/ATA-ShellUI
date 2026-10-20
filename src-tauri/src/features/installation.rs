use std::time::Duration;

use tauri::{AppHandle, Emitter};

pub async fn install_mod_inner(app: AppHandle, answered_name: &str) -> Result<(), String> {
    app.emit("setup", "Performing checks on path and name").unwrap();
    std::thread::sleep(Duration::from_secs(1));
    
    app.emit("decompression", "Decompressing mod archive").unwrap();
    std::thread::sleep(Duration::from_secs(1));

    app.emit("mod-analisys", "Analising mod files").unwrap();
    std::thread::sleep(Duration::from_secs(1));

    app.emit("conflicts-check", "Checking for conflicts with already installed mod").unwrap();
    std::thread::sleep(Duration::from_secs(1));

    app.emit("installation", "Installing the mod").unwrap();
    std::thread::sleep(Duration::from_secs(1));

    app.emit("data-update", "Updating the data file").unwrap();
    std::thread::sleep(Duration::from_secs(1));

    app.emit("success", format!("\n{}\n installed successfully", answered_name)).unwrap();
    std::thread::sleep(Duration::from_secs(1));
    
    Ok(())
}