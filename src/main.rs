use directories::ProjectDirs;
use clap::Parser;
use rusqlite::{params, Connection, Result};
use std::{fs, process::exit. path::{Path, PathBuf}};

#[derive(Parser)]
struct CLI {
    llist: String,
    path: std::path::PathBuf
}

fn getDataPath() -> Option<PathBuf> {
    if let Some(dirs) = ProjectDirs::from("com", "Wintersoft", "Note") {
        let local_path = dirs.data_local_dir();
        fs::create_dir_all(local_path);
        println!("Local data directory: {}", local_path.to_str().unwrap());
        return Option::Some(local_path.to_path_buf());
    }

    return Option::None;
}

fn connectDatabase(database_path: &PathBuf) -> Result<Connection> {
    let connection = Connection::open(database_path)?;

    connection.execute(
        "create table if not exists notes (
             id integer primary key,
             title text not null unique,
             name text not null,
             created datetime default current_timestamp,
             updated date not null
         )",
        [],
    )?;

    return Ok(connection);
}



fn main() {
    let database_path = getDataPath();
    if database_path.is_none() {
        println!("Could not find/create data folder.")
    }

    let connection = connectDatabase(&database_path.unwrap());
    //let args = CLI::parse();
}
