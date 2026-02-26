use crate::domain::{Command, OnOff, RoomSelection, SmartHome, TemperatureC};
use anyhow::{anyhow, Result};
use std::io::{self, Write};

pub enum SessionOutcome {
    Continue,
    Exit,
}

pub fn print_help() {
    println!("\n=== SmartHome Commands ===");
    println!("status");
    println!("  Zeigt den aktuellen Gesamtstatus\n");

    println!("heating set <room|all> <temp>");
    println!("  Beispiel: heating set kitchen 22.5");
    println!("heating on <room|all>");
    println!("  Beispiel: heating on all");
    println!("heating off <room|all>");
    println!("  Beispiel: heating off bedroom\n");

    println!("lights set <room|all> <on|off>");
    println!("  Beispiel: lights set living on");
    println!("lights toggle <room|all>");
    println!("  Beispiel: lights toggle bath\n");

    println!("security lock");
    println!("security unlock\n");

    println!("help");
    println!("  Zeigt diese Hilfe");
    println!("exit | quit");
    println!("  Beendet die Anwendung\n");

    println!("Räume: living, bedroom, kitchen, bath, all");
    println!(
        "Synonyme funktionieren ebenfalls: wohnzimmer, schlafzimmer, küche/kueche, bad, alle\n"
    );
}

pub fn print_welcome() {
    println!("SmartHome Controller gestartet.");
    println!("Einmal starten, danach mehrere Commands nacheinander ausführen.");
    print_help();
}

pub fn run_repl_iteration(home: &mut SmartHome) -> Result<SessionOutcome> {
    print!("smarthome> ");
    io::stdout().flush()?;

    let mut line = String::new();
    let bytes = io::stdin().read_line(&mut line)?;

    if bytes == 0 {
        return Ok(SessionOutcome::Exit);
    }

    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Ok(SessionOutcome::Continue);
    }

    if trimmed.eq_ignore_ascii_case("exit") || trimmed.eq_ignore_ascii_case("quit") {
        return Ok(SessionOutcome::Exit);
    }

    if trimmed.eq_ignore_ascii_case("help") {
        print_help();
        return Ok(SessionOutcome::Continue);
    }

    if trimmed.eq_ignore_ascii_case("status") {
        println!("{}", home.render_status());
        return Ok(SessionOutcome::Continue);
    }

    let cmd = parse_command(trimmed)?;
    home.apply(cmd);
    println!("{}", home.render_status());

    Ok(SessionOutcome::Continue)
}

fn parse_command(input: &str) -> Result<Command> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return Err(anyhow!("Leerer Command"));
    }

    match parts[0].to_lowercase().as_str() {
        "heating" => parse_heating(&parts),
        "lights" => parse_lights(&parts),
        "security" => parse_security(&parts),
        _ => Err(anyhow!(
            "Unbekannter Command. Nutze 'help' für alle verfügbaren Befehle."
        )),
    }
}

fn parse_heating(parts: &[&str]) -> Result<Command> {
    if parts.len() < 3 {
        return Err(anyhow!(
            "Ungültige Eingabe. Beispiel: heating set kitchen 22"
        ));
    }

    match parts[1].to_lowercase().as_str() {
        "set" => {
            if parts.len() != 4 {
                return Err(anyhow!(
                    "Ungültige Eingabe. Beispiel: heating set kitchen 22"
                ));
            }
            let where_ = RoomSelection::parse(parts[2])?;
            let temp_value: f32 = parts[3]
                .parse()
                .map_err(|_| anyhow!("Temperatur ist keine gültige Zahl"))?;
            let target = TemperatureC::new(temp_value)
                .ok_or_else(|| anyhow!("Ungültige Temperatur. Erlaubt: 5.0°C bis 35.0°C"))?;
            Ok(Command::HeatingSet { where_, target })
        }
        "on" => {
            if parts.len() != 3 {
                return Err(anyhow!("Ungültige Eingabe. Beispiel: heating on all"));
            }
            let where_ = RoomSelection::parse(parts[2])?;
            Ok(Command::HeatingEnabled {
                where_,
                enabled: true,
            })
        }
        "off" => {
            if parts.len() != 3 {
                return Err(anyhow!("Ungültige Eingabe. Beispiel: heating off bedroom"));
            }
            let where_ = RoomSelection::parse(parts[2])?;
            Ok(Command::HeatingEnabled {
                where_,
                enabled: false,
            })
        }
        _ => Err(anyhow!("Unbekannter heating-Befehl. Nutze: set/on/off")),
    }
}

fn parse_lights(parts: &[&str]) -> Result<Command> {
    if parts.len() < 3 {
        return Err(anyhow!(
            "Ungültige Eingabe. Beispiele: lights set living on | lights toggle all"
        ));
    }

    match parts[1].to_lowercase().as_str() {
        "set" => {
            if parts.len() != 4 {
                return Err(anyhow!("Ungültige Eingabe. Beispiel: lights set living on"));
            }
            let where_ = RoomSelection::parse(parts[2])?;
            let state = OnOff::parse(parts[3])?;
            Ok(Command::LightsSet { where_, state })
        }
        "toggle" => {
            if parts.len() != 3 {
                return Err(anyhow!("Ungültige Eingabe. Beispiel: lights toggle bath"));
            }
            let where_ = RoomSelection::parse(parts[2])?;
            Ok(Command::LightsToggle { where_ })
        }
        _ => Err(anyhow!("Unbekannter lights-Befehl. Nutze: set/toggle")),
    }
}

fn parse_security(parts: &[&str]) -> Result<Command> {
    if parts.len() != 2 {
        return Err(anyhow!(
            "Ungültige Eingabe. Beispiel: security lock | security unlock"
        ));
    }

    match parts[1].to_lowercase().as_str() {
        "lock" => Ok(Command::SecurityLockAll),
        "unlock" => Ok(Command::SecurityUnlockAll),
        _ => Err(anyhow!("Unbekannter security-Befehl. Nutze: lock/unlock")),
    }
}
