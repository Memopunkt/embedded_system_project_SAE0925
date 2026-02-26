# Datei: src/main.rs

## Zweck
Programmstart und Hauptschleife (REPL) mit Persistenz.

## Ablauf 1:1
1. Lädt Zustand via `storage::load_or_default()`.
2. Druckt Begrüßung und Status.
3. Endlosschleife:
   - Führt `cli::run_repl_iteration(&mut home)` aus.
   - Bei `Continue`: Zustand speichern.
   - Bei `Exit`: Zustand speichern, Beendenachricht, Schleife verlassen.
   - Bei Fehler: Fehlermeldung auf STDERR.
4. Gibt `Ok(())` zurück.
