use crate::tools::gui::GUI;
use crate::tools::validation::StringValidation::{Bool, Ignore, SqlTable, SqlText};
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
                    validation: SqlText,
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

        GUI::new().print_params(self as &dyn Command);

        let conn = Connection::open(DB)?;
        let result = self.add_new_entry(&conn)?;

        GUI::new().sub_title("result:").content(&result).nl();

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
                    validation: SqlText,
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

        GUI::new().print_params(self as &dyn Command);

        let mut action = String::with_capacity(21);

        let conn = Connection::open(DB)?;
        let create = self.get_param_bool("create");

        if create && !self.does_entry_exist(&conn)? {
            self.add_new_entry(&conn)?;

            action.push_str("Added new")
        } else {
            self.update_entry(&conn)?;

            match self.update_entry(&conn)? {
                true => action.push_str("Updated entry"),
                false => action.push_str("No matching entry for"),
            }
        }

        GUI::new()
            .sub_title("result:")
            .content(&format!(
                "Success: {result} for {index}->{key}",
                result = action,
                index = self.get_param("index"),
                key = self.get_param("key")
            ))
            .nl();

        Ok(())
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

        GUI::new().print_params(self as &dyn Command);

        let conn = Connection::open(DB)?;

        let row_existed = self.remove_entry(&conn)?;

        let result = match row_existed {
            true => format!(
                "Success: Row removed with key '{key}'",
                key = self.get_param("key")
            ),
            false => format!(
                "Warning: Row did not exist with key '{key}'",
                key = self.get_param("key")
            ),
        };

        GUI::new().sub_title("result:").content(&result).nl();

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

    fn add_new_entry(&self, conn: &Connection) -> Result<String> {
        match conn.execute(
            &format!(
                "INSERT INTO {table} (`key`, `data`) VALUES (?1, ?2)",
                table = self.get_param("index")
            ),
            [self.get_param("key"), self.get_param("data")],
        ) {
            Err(err) => Ok("Error: ".to_string() + &err.to_string()),
            _ => Ok(format!(
                "Success: Entry added for {index}->{key}",
                index = self.get_param("index"),
                key = self.get_param("key"),
            )),
        }
    }

    fn remove_entry(&self, conn: &Connection) -> Result<bool> {
        let rows_effected = conn.execute(
            &format!(
                "DELETE FROM {table} WHERE `key` = ?1",
                table = self.get_param("index")
            ),
            [self.get_param("key")],
        )?;

        Ok(rows_effected > 0)
    }

    fn update_entry(&self, conn: &Connection) -> Result<bool> {
        let updated_users = conn.execute(
            &format!(
                "UPDATE {table} SET `data` = ?1 WHERE `key` = ?2",
                table = self.get_param("index")
            ),
            [self.get_param("data"), self.get_param("key")],
        )?;

        Ok(updated_users > 0)
    }
}
