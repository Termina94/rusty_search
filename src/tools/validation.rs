use anyhow::{bail, Result};

pub enum StringValidation {
    SqlTable,
    SqlColumn,
    Bool,
    Ignore,
}

pub fn validate_string(value: &String, validation: StringValidation) -> Result<()> {
    match validation {
        StringValidation::SqlTable => sql_table(value),
        StringValidation::SqlColumn => sql_column(value),
        StringValidation::Bool => boolean(value),
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

pub fn boolean(name: &String) -> Result<()> {
    match name.as_str() {
        "true" | "false" => Ok(()),
        _ => bail!("Invalid value for boolean, expected true|false"),
    }
}
