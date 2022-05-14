use anyhow::{Ok, Result};
use commands::help::Help;
use commands::manage::Manage;
use traits::command::Command;

use std::env;
mod commands {
    pub mod help;
    pub mod manage;
}
mod tools {
    pub mod debug;
}
pub mod traits {
    pub mod command;
}

pub const EMPTY: String = String::new();

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    run_command(args)?;

    Ok(())
}

fn run_command(args: Vec<String>) -> Result<()> {
    let empty = String::with_capacity(0);
    let command = args.get(1).unwrap_or(&empty);
    let action = args.get(2).unwrap_or(&empty);
    let params = match args.len() >= 4 {
        true => &args[3..],
        false => &[] as &[String],
    };

    let mut route: Box<dyn Command> = match command.as_str() {
        "manage" => Box::new(Manage::new()),
        _ => Box::new(Help::new()),
    };

    route.run(action, params)?;

    Ok(())
}
