# Datei: src/domain.rs

## Zweck
Enthält den kompletten fachlichen Zustand und alle Zustandsänderungen.

## Inhalt 1:1
- Raumtypen:
  - `Room` mit vier Räumen
  - `RoomSelection` (`One`, `All`)
- Zustands-/Hilfstypen:
  - `OnOff`
  - `TemperatureC` mit Grenzenprüfung (`5.0..=35.0`)
- Command-Typ:
  - `Command` mit Varianten für Heizung/Licht/Sicherheit
- Interne Zustandsstrukturen:
  - `HeatingState`, `LightState`
  - `HeatingSystem`, `LightingSystem`, `SecuritySystem`
- Hauptaggregat:
  - `SmartHome` mit Feldern `heating`, `lighting`, `security`
- zentrale Methoden:
  - `SmartHome::new_default()`
  - `SmartHome::apply(cmd)`
  - `SmartHome::render_status()`

## Verhalten 1:1
- `HeatingSet` setzt Zieltemperatur und aktiviert Heizung im Ziel.
- `HeatingEnabled` setzt nur den `enabled`-Status.
- `LightsSet` setzt Licht auf an/aus.
- `LightsToggle` invertiert den Lichtstatus.
- `SecurityLockAll`/`SecurityUnlockAll` setzen `locked`.
