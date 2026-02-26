mod cli;
mod domain;
mod storage;

use crate::cli::SessionOutcome;
use anyhow::Result;

fn main() -> Result<()> {
    let mut home = storage::load_or_default()?;

    cli::print_welcome();
    println!("{}", home.render_status());

    loop {
    match cli::run_repl_iteration(&mut home) {
        
        // Der Zustand wird jetzt nur gespeichert, wenn sich wirklich etwas geändert hat.
        // Dadurch werden unnötige Schreibzugriffe auf die Datei vermieden
        // (wichtig für Performance und für den Einsatz auf dem Raspberry Pi).
        Ok(SessionOutcome::Continue { changed }) => {
            if changed {
                storage::save(&home)?;
            }
        }
        Ok(SessionOutcome::Exit) => {
            storage::save(&home)?;
            println!("SmartHome Controller beendet.");
            break;
        }
        Err(err) => {
            eprintln!("Fehler: {err}");
        }
    }
}

    Ok(())
}


