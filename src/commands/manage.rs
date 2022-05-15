use crate::tools::validation::StringValidation::{Ignore, SqlColumn, SqlTable};
use crate::traits::command::{derive_getters, ParamRule};
use crate::traits::command::{Command, Runnable};
use anyhow::{Ok, Result};
use rusqlite::Connection;
use std::collections::HashMap;

use std::fs::remove_file;
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
    Delete,
    #[strum(ascii_case_insensitive)]
    Purge,
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
            Actions::Create => self.create_table(params)?,
            Actions::Init => self.init_index(params)?,
            Actions::Help => self.help()?,
            Actions::Delete => self.delete_index(params)?,
            Actions::Purge => Manage::purge_db()?,
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

    fn create_table(&mut self, params: &[String]) -> Result<()> {
        self.assert_params(
            vec![ParamRule {
                key: "index",
                validation: SqlTable,
                required: &true,
            }],
            params,
        )?;

        let connection = Connection::open(DB)?;

        connection.execute_batch(&format!(
            "DROP TABLE if exists `{table}`;
            CREATE TABLE `{table}` (
                `key` TEXT PRIMARY KEY,
                `data` TEXT);
            ",
            table = self.get_param("index")
        ))?;

        println!(
            "
        SUCCESS!\n\r"
        );

        Ok(())
    }

    fn init_index(&mut self, params: &[String]) -> Result<()> {
        self.assert_params(
            vec![
                ParamRule {
                    key: "index",
                    validation: SqlTable,
                    required: &true,
                },
                ParamRule {
                    key: "key",
                    validation: SqlColumn,
                    required: &true,
                },
                ParamRule {
                    key: "data",
                    validation: Ignore,
                    required: &true,
                },
            ],
            params,
        )?;

        self.create_table(params)?;

        println!(
            "
        Running 'Init' command with params:\n
            index:      {}
            key:        {}
            data:       {}\n",
            self.get_param("index"),
            self.get_param("key"),
            self.get_param("data"),
        );

        let conn = Connection::open(DB)?;
        let mut stmt = conn.prepare(&format!(
            "INSERT INTO `{table}` (`key`, `data`) values (?1, ?2)",
            table = self.get_param("index")
        ))?;

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

    fn delete_index(&mut self, params: &[String]) -> Result<()> {
        self.assert_params(
            vec![ParamRule {
                key: "index",
                validation: SqlTable,
                required: &true,
            }],
            params,
        )?;

        println!(
            "
        Running 'Delete' command with params:\n
            index:      {}\n",
            self.get_param("index"),
        );

        let conn = Connection::open(DB)?;
        conn.execute(
            &format!(
                "DROP TABLE IF EXISTS `{table}`",
                table = self.get_param("index")
            ),
            [],
        )?;

        Ok(())
    }

    fn purge_db() -> Result<()> {
        remove_file("db.db")?;

        println!(
            "
        SUCCESS!\n\r"
        );

        Ok(())
    }
}
