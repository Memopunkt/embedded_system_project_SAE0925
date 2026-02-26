# Datei: src/storage.rs

## Zweck
Laden und Speichern des `SmartHome`-Zustands als JSON-Datei.

## Inhalt 1:1
- Konstante Datei: `STATE_FILE = "smarthome_state.json"`
- `load_or_default()`:
  - Wenn Datei fehlt: `SmartHome::new_default()`
  - Sonst: Datei lesen und JSON zu `SmartHome` deserialisieren
- `save(home)`:
  - Zustand zu formatiertem JSON serialisieren
  - JSON nach `smarthome_state.json` schreiben

## Fehlerbehandlung
- Nutzt `anyhow::Context` für konkrete Fehlermeldungen beim Lesen, Parsen, Serialisieren und Schreiben.
