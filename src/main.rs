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
            Ok(SessionOutcome::Continue) => {
                storage::save(&home)?;
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
