use anyhow::{bail, Ok, Result};
use commands::help::Help;
use commands::manage::Manage;
use strum::{Display, EnumString};
use traits::command::Command;

use std::{env, str::FromStr};
mod commands {
    pub mod help;
    pub mod manage;
}
mod tools {
    pub mod debug;
    pub mod gui;
    pub mod validation;
}
pub mod traits {
    pub mod command;
}

#[derive(Display, EnumString)]
enum Commands {
    #[strum(ascii_case_insensitive)]
    Manage,
    #[strum(ascii_case_insensitive)]
    Update,
    #[strum(ascii_case_insensitive)]
    Search,
    #[strum(ascii_case_insensitive)]
    Rank,
    #[strum(ascii_case_insensitive)]
    Config,
    #[strum(ascii_case_insensitive)]
    Help,
}

pub const EMPTY: String = String::new();

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    run_command(args)?;

    Ok(())
}

fn run_command(args: Vec<String>) -> Result<()> {
    let empty = String::with_capacity(0);
    let command_string = args.get(1).unwrap_or(&empty);
    let action = args.get(2).unwrap_or(&empty);
    let params = match args.len() >= 4 {
        true => &args[3..],
        false => &[] as &[String],
    };

    let command = Commands::from_str(command_string).unwrap_or(Commands::Help);

    let mut route: Box<dyn Command> = match command {
        Commands::Manage => Box::new(Manage::new()),
        Commands::Help => Box::new(Help::new()),
        _ => bail!(format!("Nothing implemented for {}", command)),
    };

    route.run(action, params)?;

    Ok(())
}
