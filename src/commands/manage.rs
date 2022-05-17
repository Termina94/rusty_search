use crate::tools::gui::GUI;
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

impl Manage {}

impl Command for Manage {
    derive_getters!();

    fn help(&self) -> Result<()> {
        GUI::new()
            .title("Manage Command")
            .nl()
            .sub_title("actions:")
            .nl()
            .content("create: {index} {key}             | create table called {index} and set {key}")
            .content("init:   {index} {key} {data}      | Create table called {index}, set {key} and populate with {data}")
            .content("delete: {index}                   | Delete index and data")
            .content("purge:                            | Delete database")
            .nl();

        Ok(())
    }
}

impl Runnable for Manage {
    fn run(&mut self, action: &String, params: &[String]) -> Result<()> {
        let action = Actions::from_str(action).unwrap_or(Actions::Help);

        match action {
            Actions::Create => self.create_table(params, true)?,
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

    fn create_table(&mut self, params: &[String], output: bool) -> Result<()> {
        self.assert_params(
            vec![ParamRule {
                key: "index",
                validation: SqlTable,
                required: &true,
            }],
            params,
        )?;

        if output {
            GUI::new()
                .title("Running 'Create' command:")
                .nl()
                .sub_title("params")
                .nl()
                .content(&format!("index: {}", self.get_param("index")))
                .nl();
        }

        let connection = Connection::open(DB)?;

        connection.execute_batch(&format!(
            "DROP TABLE if exists `{table}`;
            CREATE TABLE `{table}` (
                `key` TEXT PRIMARY KEY,
                `data` TEXT);
            ",
            table = self.get_param("index")
        ))?;

        if output {
            GUI::new()
                .sub_title("result:")
                .content(&format!(
                    "Success: `{}` Index Created/Exists",
                    self.get_param("index")
                ))
                .nl();
        }

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

        self.create_table(params, false)?;

        GUI::new()
            .title("Running 'Init' command:")
            .nl()
            .sub_title("params:")
            .nl()
            .content(&format!("index:   {}", self.get_param("index")))
            .content(&format!("key:     {}", self.get_param("key")))
            .content(&format!("data:    {}", self.get_param("data")))
            .nl();

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

        GUI::new().sub_title("result:").content("Success").nl();

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

        GUI::new()
            .title("Running 'Delete' command")
            .nl()
            .sub_title("params:")
            .nl()
            .content(&format!("index: {}", self.get_param("index")))
            .nl();

        let conn = Connection::open(DB)?;
        conn.execute(
            &format!(
                "DROP TABLE IF EXISTS `{table}`",
                table = self.get_param("index")
            ),
            [],
        )?;

        GUI::new().sub_title("result:").content("Success").nl();

        Ok(())
    }

    fn purge_db() -> Result<()> {
        GUI::new().title("Running 'Purge'").nl();

        remove_file("db.db")?;

        GUI::new().sub_title("result:").content("Success").nl();

        Ok(())
    }
}
