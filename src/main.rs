use anyhow::Result;

use std::env;
use traits::route::Init;
use traits::route::Route;

mod tools;
mod traits;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    run_command(args)?;

    Ok(())
}

fn run_command(args: Vec<String>) -> Result<()> {
    let empty_command = String::new();
    let command = args.get(1).unwrap_or(&empty_command);

    // TODO: Make a help route that is used on empty command

    let mut route = match command.as_str() {
        "init" => Init::default(),
        _ => Init::default(),
    };

    route.setup(args)?;

    route.execute()?;

    Ok(())
}
