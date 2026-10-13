use std::{io, process::Command};

pub fn launch_automata() -> Result<(), String> {
    launch_automata_inner().map_err(|err| err.to_string())
}
fn launch_automata_inner() -> Result<(), io::Error> {
    let status = Command::new("steam://rungameid/524220")
        .status()?;

    println!(status);

    Ok(())
}