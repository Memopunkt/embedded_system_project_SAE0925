# TechnischeGesamtdokumentation.md

## 1. Projektziel
Das Projekt ist ein terminalbasiertes SmartHome-System in Rust.
Implementiert sind:
- Heizung pro Raum
- Licht pro Raum
- globale Hausverriegelung

Nicht implementiert:
- Garagentor-Steuerung

## 2. Technische Architektur
### 2.1 Modulaufteilung
- `src/main.rs`
  - Einstiegspunkt
  - REPL-Hauptschleife
  - Speichern nach jeder erfolgreichen Eingabe
- `src/cli.rs`
  - Prompt, Hilfe, Benutzerinput
  - Parsing von Text in fachliche Commands
- `src/domain.rs`
  - vollständiges Domänenmodell
  - Fachregeln und Zustandsänderungen
  - Statusdarstellung
- `src/storage.rs`
  - Laden/Speichern des Zustands in JSON

### 2.2 Datenfluss
1. Benutzer gibt Command im Terminal ein.
2. `cli` parst Eingabe zu `Command`.
3. `domain` führt Änderung über `SmartHome::apply` aus.
4. `main` speichert den Zustand in `smarthome_state.json`.
5. Status wird auf der Konsole ausgegeben.

## 3. Befehle (CLI)
### 3.1 Allgemein
- `help`
- `status`
- `exit` / `quit`

### 3.2 Heizung
- `heating set <room|all> <temp>`
- `heating on <room|all>`
- `heating off <room|all>`

Regeln:
- Temperaturbereich: `5.0` bis `35.0` (inklusive)
- `heating set` setzt Zieltemperatur und aktiviert Heizung automatisch

### 3.3 Licht
- `lights set <room|all> <on|off>`
- `lights toggle <room|all>`

### 3.4 Sicherheit
- `security lock`
- `security unlock`

Hinweis:
- `lock` ist aktuell ein Statusflag und blockiert keine anderen Commands.

## 4. Räume und Synonyme
Primär:
- `living`, `bedroom`, `kitchen`, `bath`, `all`

Akzeptierte Synonyme:
- `wohnzimmer`, `schlafzimmer`, `küche`/`kueche`, `bad`, `alle`

## 5. Persistenz und Dateiformat
### 5.1 Persistenzdatei
- Datei: `smarthome_state.json`
- Beim Start: Laden, falls vorhanden
- Sonst: Default-Zustand
- Nach erfolgreichem Command: Speichern

### 5.2 JSON-Struktur
- `heating.per_room[Raum].enabled`
- `heating.per_room[Raum].target`
- `lighting.per_room[Raum].on`
- `security.locked`

## 6. Fehlerbehandlung
- Ungültige Befehle oder Parameter erzeugen Fehlermeldungen.
- Programm läuft weiter, Zustand bleibt bei Fehlern unverändert.
- JSON-Lade-/Speicherfehler werden mit Kontext gemeldet (`anyhow::Context`).

## 7. Datei-für-Datei (1:1)
### 7.1 `Cargo.toml`
- Definiert Paketdaten, Dependencies und Binärziel `smarthome`.
- Abhängigkeiten: `anyhow`, `serde`, `serde_json`.

### 7.2 `Cargo.lock`
- Fixiert aufgelöste Abhängigkeitsversionen für reproduzierbare Builds.

### 7.3 `.gitignore`
- Ignoriert Build-, IDE- und OS-Artefakte (`target`, `.idea`, `.vscode`, `.DS_Store`, etc.).

### 7.4 `src/main.rs`
- Startet Anwendung, lädt Zustand, führt REPL aus, speichert Zustand, beendet sauber.

### 7.5 `src/cli.rs`
- Enthält Hilfeausgabe, REPL-Einleselogik und Parser für `heating/lights/security`.

### 7.6 `src/domain.rs`
- Enthält:
  - Raumtypen (`Room`, `RoomSelection`)
  - Zustands-/Hilfstypen (`OnOff`, `TemperatureC`)
  - Commands (`Command`)
  - Systeme (`HeatingSystem`, `LightingSystem`, `SecuritySystem`)
  - Aggregat `SmartHome` mit `apply` und `render_status`

### 7.7 `src/storage.rs`
- Lädt/speichert `SmartHome` als JSON in `smarthome_state.json`.

### 7.8 `smarthome_state.json`
- Persistenter Laufzeitzustand des SmartHome-Systems.

## 8. Scope-Abgrenzung
Folgende Themen sind aktuell nicht implementiert und daher kein Funktionsbestandteil:
- Garagentor-Steuerung
- Garagentor-Sicherheitsprotokoll

## 9. Build-Status
Technischer Stand wurde mit `cargo check` validiert.
