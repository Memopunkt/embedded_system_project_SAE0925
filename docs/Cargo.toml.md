# Datei: Cargo.toml

## Zweck
Definiert Paket-Metadaten, Abhängigkeiten und Binärziel.

## Inhalt 1:1
- Paketname: `embedded_system_project`
- Version: `0.1.0`
- Edition: `2021`
- Autorenfelder gesetzt
- Beschreibung und Lizenz gesetzt
- Dependencies:
  - `anyhow`
  - `serde` mit `derive`
  - `serde_json`
- Release-Profil:
  - `lto = true`
  - `codegen-units = 1`
  - `panic = abort`
- Binärziel:
  - Name `smarthome`
  - Entry-Point `src/main.rs`
