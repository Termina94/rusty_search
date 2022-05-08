use serde::{Deserialize, Serialize};
use std::{env, error::Error};

use crate::tools::tool::assert_args;

mod tools;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    run_command(args)?;

    Ok(())
}

fn run_command(args: Args) -> Result<()> {
    match args.get(1) {
        Some(command) => match command.as_str() {
            "init" => init(args)?,
            // "search" => search(args)?,
            _ => println!("Command Not Found"),
        },
        None => print!("Please Enter A Command: TODO: Show Help"),
    }

    Ok(())
}

/**
 * ARGS MANAGER
 * ArgBox::new()
 *  .add("index", required-> true, default-> "")print
 *
 */

#[derive(Serialize, Deserialize, Debug)]
struct Product {
    name: String,
}

fn init(args: Args) -> Result<()> {
    let map = assert_args(["index".to_string(), "data".to_string()], args);

    println!(
        "
        Running 'init' command with args:\n
            index:      {}
            data:       {}\n",
        map.get("index").unwrap(),
        map.get("data").unwrap()
    );

    let mut connection = sqlite::open("db.db")?;

    let products: Vec<Product> = serde_json::from_str(map.get("data").unwrap()).unwrap();

    let mut stmt = connection
        .prepare("INSERT INTO products (`key`, `data`) values (?, ?)")
        .unwrap();

    stmt.execute(["test", map.get("data").unwrap()]).unwrap();

    connection
        .execute(
            "
                DROP TABLE if exists products;
                CREATE TABLE products (`id` INTEGER, `key` TEXT, `data` TEXT);
            ",
        )
        .unwrap();

    Ok(())
}

// fn search(args: Args) -> Result<()> {
//     let [index, term, options]: [String; 3] =
//         tool::destruct_array_fill(args, 2..=4).try_into().unwrap();

//     println!(
//         "
//         Running 'search' command with args:\n
//             index:      {}
//             term:       {}
//             options:    {}\n",
//         index, term, options
//     );

//     Ok(())
// }

type Args = Vec<String>;
type Result<T> = std::result::Result<T, Box<dyn Error>>;
