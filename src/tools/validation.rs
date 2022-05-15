use anyhow::{bail, Result};

pub enum StringValidation {
    SqlTable,
    SqlColumn,
    Ignore,
}

pub fn validate_string(value: &String, validation: StringValidation) -> Result<()> {
    match validation {
        StringValidation::SqlTable => sql_table(value),
        StringValidation::SqlColumn => sql_column(value),
        StringValidation::Ignore => Ok(()),
    }
}

pub fn sql_table(name: &String) -> Result<()> {
    for char in name.chars() {
        if !char.is_alphabetic() {
            bail!("Invalid value for index/table");
        }
    }
    Ok(())
}

pub fn sql_column(name: &String) -> Result<()> {
    for char in name.chars() {
        if !char.is_alphabetic() {
            bail!("Invalid value for key/column");
        }
    }
    Ok(())
}
