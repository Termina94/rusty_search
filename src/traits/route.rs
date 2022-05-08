use std::collections::HashMap;

use anyhow::{bail, Result};
use rusqlite::Connection;

const DB: &str = "db.db";

#[derive(Default)]
pub struct Init {
    pub required_args: Vec<String>,
    pub args: HashMap<String, String>,
}

impl Init {
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

        Ok(())
    }

    fn init(&self) -> Result<()> {
        self.create_table()?;

        println!(
            "
        Running 'init' command with args:\n
            index:      {}
            data:       {}\n",
            self.get_arg("index"),
            self.get_arg("data"),
        );

        let conn = Connection::open(DB)?;
        let mut stmt = conn.prepare("INSERT INTO products (`key`, `data`) values (?1, ?2)")?;

        let products: Vec<HashMap<String, String>> = serde_json::from_str(self.get_arg("data"))?;

        for prod in products {
            stmt.execute([
                &prod.get(self.get_arg("key")).unwrap(),
                &serde_json::to_string(&prod).unwrap(),
            ])?;
        }

        Ok(())
    }
}

impl Route for Init {
    fn setup(&mut self, args: Vec<String>) -> Result<&mut Self> {
        self.require("index")
            .require("key")
            .require("data")
            .assert_args(args)
    }

    fn require(&mut self, arg: &str) -> &mut Self {
        self.required_args.push(arg.to_string());
        self
    }

    fn required_args(&self) -> &Vec<String> {
        &self.required_args
    }

    fn set_args(&mut self, args: HashMap<String, String>) {
        self.args = args;
    }

    fn get_args(&self) -> &HashMap<String, String> {
        &self.args
    }

    fn execute(&self) -> Result<()> {
        self.init()?;
        Ok(())
    }
}

pub trait Route {
    fn setup(&mut self, args: Vec<String>) -> Result<&mut Self>;
    fn require(&mut self, arg: &str) -> &mut Self;
    fn required_args(&self) -> &Vec<String>;
    fn set_args(&mut self, args: HashMap<String, String>);
    fn get_args(&self) -> &HashMap<String, String>;
    fn get_arg(&self, key: &str) -> &String {
        self.get_args().get(key).expect("Arg key does not exists")
    }
    fn assert_args(&mut self, args: Vec<String>) -> Result<&mut Self> {
        let mut map: HashMap<String, String> = HashMap::default();

        for (i, arg) in self.required_args().iter().enumerate() {
            if let Some(value) = args.get(i + 2) {
                map.insert(arg.into(), value.into());
            } else {
                bail!("fucked it mate")
            }
        }

        self.set_args(map);

        Ok(self)
    }
    fn execute(&self) -> Result<()>;
}
