mod action;
mod alter;
mod command;
mod create;
mod file;
mod templates;

use alter::Alter;
use anyhow::Ok;

fn main() -> Result<(), anyhow::Error> {
    let alter = Alter::new();
    alter.init();

    Ok(())
}
