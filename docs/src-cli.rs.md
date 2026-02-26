# Datei: src/cli.rs

## Zweck
Textbasierte Ein-/Ausgabe und Parsing von Benutzerbefehlen.

## Inhalt 1:1
- Definiert `SessionOutcome` mit `Continue` und `Exit`.
- `print_help()` druckt alle erlaubten Commands.
- `print_welcome()` druckt Starttext und Hilfe.
- `run_repl_iteration(home)`:
  - zeigt Prompt `smarthome> `
  - liest eine Zeile
  - behandelt Spezialfälle: EOF, leer, `exit/quit`, `help`, `status`
  - parst sonst in `Command`
  - ruft `home.apply(cmd)` auf
  - druckt Status
- `parse_command(input)` delegiert nach Präfix:
  - `heating` -> `parse_heating`
  - `lights` -> `parse_lights`
  - `security` -> `parse_security`
- `parse_heating(parts)` unterstützt:
  - `set`, `on`, `off`
- `parse_lights(parts)` unterstützt:
  - `set`, `toggle`
- `parse_security(parts)` unterstützt:
  - `lock`, `unlock`
