use crate::traits::command::derive_getters;
use crate::traits::command::{Command, Runnable};
use anyhow::Result;
use std::collections::HashMap;

pub struct Help {
    pub required_args: Vec<String>,
    pub params: HashMap<String, String>,
}

impl Command for Help {
    derive_getters!();

    fn help(&self) -> Result<()> {
        Ok(())
    }
}

impl Runnable for Help {
    fn run(&mut self, _action: &String, _params: &[String]) -> Result<()> {
        println!(
            r#"
        ### Welcome to Rusty Search ###
        
        commands:

            init: {{index}} {{key}} {{data}}  | Create table called {{index}}, set {{key}} and populate with {{data}}
            help:                       | Display helpful tips and commands

            Run `{{command}} help` for specific help
        "#
        );

        Ok(())
    }
}

impl Help {
    pub fn new() -> Self {
        Self {
            required_args: Vec::new(),
            params: HashMap::default(),
        }
    }
}
