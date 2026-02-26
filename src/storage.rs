use crate::domain::SmartHome;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

const STATE_FILE: &str = "smarthome_state.json";

pub fn load_or_default() -> Result<SmartHome> {
    if !Path::new(STATE_FILE).exists() {
        return Ok(SmartHome::new_default());
    }

    let json = fs::read_to_string(STATE_FILE).context("Konnte State-Datei nicht lesen")?;
    let home: SmartHome = serde_json::from_str(&json).context("State-Datei ist ungültiges JSON")?;
    Ok(home)
}

pub fn save(home: &SmartHome) -> Result<()> {
    let json = serde_json::to_string_pretty(home).context("Konnte State nicht serialisieren")?;
    fs::write(STATE_FILE, json).context("Konnte State-Datei nicht schreiben")?;
    Ok(())
}
