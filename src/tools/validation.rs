use anyhow::{bail, Result};

use crate::traits::command::ParamRule;

pub enum StringValidation {
    SqlTable,
    SqlColumn,
    Bool,
    Ignore,
}

pub fn validate_string(value: &String, rule: &ParamRule) -> Result<()> {
    match rule.validation {
        StringValidation::SqlTable => sql_table(value),
        StringValidation::SqlColumn => sql_column(value),
        StringValidation::Bool => boolean(value, rule.key),
        StringValidation::Ignore => Ok(()),
    }
}

pub fn sql_table(name: &String) -> Result<()> {
    for char in name.chars() {
        if !char.is_alphabetic() {
            bail!("Invalid value for index, expected alphabetic string");
        }
    }
    Ok(())
}

pub fn sql_column(name: &String) -> Result<()> {
    for char in name.chars() {
        if !char.is_alphabetic() {
            bail!("Invalid value for key, expected alphabetic string");
        }
    }
    Ok(())
}

pub fn boolean(name: &String, key: &str) -> Result<()> {
    match name.as_str() {
        "true" | "false" => Ok(()),
        _ if name == key => Ok(()),
        _ => bail!("Invalid value for boolean, expected true|false"),
    }
}
