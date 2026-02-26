# Datei: smarthome_state.json

## Zweck
Persistenter Laufzeitzustand des SmartHome-Systems.

## Inhalt 1:1
- Root-Objekte:
  - `heating.per_room`
  - `lighting.per_room`
  - `security.locked`
- `heating.per_room` enthält pro Raum:
  - `enabled` (bool)
  - `target` (float)
- `lighting.per_room` enthält pro Raum:
  - `on` (bool)
- `security.locked` enthält:
  - `true` oder `false`

## Nutzung
- Wird beim Start geladen (`src/storage.rs`)
- Wird nach Commands wieder geschrieben (`src/storage.rs`)
