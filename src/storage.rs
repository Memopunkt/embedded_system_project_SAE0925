use crate::domain::SmartHome;
use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::Path;

const STATE_FILE: &str = "smarthome_state.json";

pub fn save(home: &SmartHome) -> Result<()> {
    // 1) serialize
    let json = serde_json::to_vec_pretty(home).context("Konnte State nicht serialisieren")?;

    // 2) write to tmp file
    let tmp_file = format!("{STATE_FILE}.tmp");
    {
        let mut f = fs::File::create(&tmp_file).context("Konnte tmp-State-Datei nicht erstellen")?;
        f.write_all(&json).context("Konnte tmp-State-Datei nicht schreiben")?;
        let _ = f.sync_all(); // nice-to-have, не ломает ничего
    }

    // 3) optional backup
    if Path::new(STATE_FILE).exists() {
        let _ = fs::copy(STATE_FILE, format!("{STATE_FILE}.bak"));
    }

    // 4) atomic replace
    fs::rename(&tmp_file, STATE_FILE).context("Konnte State-Datei nicht atomar ersetzen")?;
    Ok(())
}

pub fn load() -> anyhow::Result<crate::domain::SmartHome> {
    let json = fs::read_to_string(STATE_FILE).context("Konnte State-Datei nicht lesen")?;
    serde_json::from_str(&json).context("Konnte State-Datei nicht deserialisieren")
}

pub fn load_or_default() -> anyhow::Result<crate::domain::SmartHome> {
    load().or_else(|_| Ok(crate::domain::SmartHome::new_default()))
}
