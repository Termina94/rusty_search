use crate::tools::gui::GUI;
use crate::tools::validation::StringValidation::{Bool, Ignore, SqlColumn, SqlTable};
use crate::traits::command::{derive_getters, ParamRule};
use crate::traits::command::{Command, Runnable};
use anyhow::{Ok, Result};
use rusqlite::Connection;
use std::collections::HashMap;
use std::str::FromStr;
use strum_macros::{Display, EnumString};

const DB: &str = "db.db";

pub struct Edit {
    pub params: HashMap<String, String>,
}

#[derive(Display, EnumString, Debug)]
enum Actions {
    #[strum(ascii_case_insensitive)]
    Add,
    #[strum(ascii_case_insensitive)]
    Update,
    #[strum(ascii_case_insensitive)]
    Remove,
    #[strum(ascii_case_insensitive)]
    Help,
}

impl Edit {}

impl Command for Edit {
    derive_getters!();

    fn help(&self) -> Result<()> {
        GUI::new()
            .title("Edit Command")
            .nl()
            .sub_title("actions:")
            .nl()
            .content("create: {index} {key} {data}              | Create a new entry in {index} with {key} and {data}")
            .content("update: {index} {key} {data} {?create}    | Update entry in {index} that matches {key} with {data}")
            .content("remove: {index} {key}                     | Delete index and data")
            .nl()
            .content("* {?create} is an optional bool to create a new entry on no key match")
            .nl();

        Ok(())
    }
}

impl Runnable for Edit {
    fn run(&mut self, action: &String, params: &[String]) -> Result<()> {
        let action = Actions::from_str(action).unwrap_or(Actions::Help);

        match action {
            Actions::Add => self.add(params)?,
            Actions::Update => self.update(params)?,
            Actions::Remove => self.remove(params)?,
            Actions::Help => self.help()?,
        }

        Ok(())
    }
}

impl Edit {
    pub fn new() -> Self {
        Self {
            params: HashMap::default(),
        }
    }

    fn add(&mut self, params: &[String]) -> Result<()> {
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

        let conn = Connection::open(DB)?;
        self.add_new_entry(&conn)?;

        Ok(())
    }

    fn update(&mut self, params: &[String]) -> Result<()> {
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
                ParamRule {
                    key: "create",
                    validation: Bool,
                    required: &false,
                },
            ],
            params,
        )?;

        let conn = Connection::open(DB)?;
        let create = self.get_param_bool("create");

        if create && !self.does_entry_exist(&conn)? {
            println!("doing!!");
            return self.add_new_entry(&conn);
        }

        self.update_entry(&conn)
    }

    fn remove(&mut self, params: &[String]) -> Result<()> {
        self.assert_params(
            vec![
                ParamRule {
                    key: "index",
                    validation: SqlTable,
                    required: &true,
                },
                ParamRule {
                    key: "key",
                    validation: Ignore,
                    required: &true,
                },
            ],
            params,
        )?;

        // TODO:
        //      query to remove row from index table
        //      add gui feedback for this commands actions
        //      ensure help command reflects this new command correctly
        //      remove SqlColumn from validation as the key param is the column value, not column name

        Ok(())
    }
}

impl Edit {
    fn does_entry_exist(&self, conn: &Connection) -> Result<bool> {
        let result: bool = conn.query_row(
            &format!(
                "SELECT count(*) FROM `{table}` WHERE `key` = ?1",
                table = self.get_param("index"),
            ),
            [self.get_param("key")],
            |row| row.get(0),
        )?;

        Ok(result)
    }

    fn add_new_entry(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            &format!(
                "INSERT INTO {table} (`key`, `data`) VALUES (?1, ?2)",
                table = self.get_param("index")
            ),
            [self.get_param("key"), self.get_param("data")],
        )?;

        Ok(())
    }

    fn update_entry(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            &format!(
                "UPDATE {table} SET `data` = ?1 WHERE `key` = ?2",
                table = self.get_param("index")
            ),
            [self.get_param("data"), self.get_param("key")],
        )?;

        Ok(())
    }
}
