use crate::traits::command::derive_getters;
use crate::traits::command::{Command, Runnable};
use anyhow::{Ok, Result};
use rusqlite::Connection;
use std::collections::HashMap;
use std::str::FromStr;
use strum_macros::{Display, EnumString};

const DB: &str = "db.db";

pub struct Manage {
    pub params: HashMap<String, String>,
}

#[derive(Display, EnumString, Debug)]
enum Actions {
    #[strum(ascii_case_insensitive)]
    Create,
    #[strum(ascii_case_insensitive)]
    Init,
    #[strum(ascii_case_insensitive)]
    Help,
}

#[derive(Display, EnumString)]
enum Params {
    Index,
    Key,
    Data,
}

impl Manage {}

impl Command for Manage {
    derive_getters!();

    fn help(&self) -> Result<()> {
        println!(
            r#"
        ### Manage Command ###
        
        actions:

            create: {{index}} {{key}}           | create table called {{index}} and set {{key}}
            init:   {{index}} {{key}} {{data}}  | Create table called {{index}}, set {{key}} and populate with {{data}}
            help:                               | Display helpful tips and commands

            Run `{{command}} help` for specific help
        "#
        );
        Ok(())
    }
}

impl Runnable for Manage {
    fn run(&mut self, action: &String, params: &[String]) -> Result<()> {
        let action = Actions::from_str(action).unwrap_or(Actions::Help);

        match action {
            Actions::Create => self.create_table()?,
            Actions::Init => self.init_index(params)?,
            Actions::Help => self.help()?,
        }

        Ok(())
    }
}

impl Manage {
    pub fn new() -> Self {
        Self {
            params: HashMap::default(),
        }
    }

    fn create_table(&self) -> Result<()> {
        let connection = Connection::open(DB)?;

        connection.execute_batch(
            "
            DROP TABLE if exists products;
            CREATE TABLE products (
                `key` TEXT PRIMARY KEY,
                `data` TEXT);
        ",
        )?;

        println!(
            "
        SUCCESS!\n\r"
        );

        Ok(())
    }

    fn init_index(&mut self, params: &[String]) -> Result<()> {
        self.assert_params(vec!["index", "key", "data"], params)?;

        self.create_table()?;

        println!(
            "
        Running 'init' command with params:\n
            index:      {}
            key:        {}\n
            data:       {}\n",
            self.get_param("index"),
            self.get_param("key"),
            self.get_param("data"),
        );

        let conn = Connection::open(DB)?;
        let mut stmt = conn.prepare("INSERT INTO products (`key`, `data`) values (?1, ?2)")?;

        let products: Vec<HashMap<String, String>> = serde_json::from_str(self.get_param("data"))?;

        for prod in products {
            stmt.execute([
                &prod.get(self.get_param("key")).unwrap(),
                &serde_json::to_string(&prod).unwrap(),
            ])?;
        }

        println!(
            "
        SUCCESS!\n\r"
        );

        Ok(())
    }
}
