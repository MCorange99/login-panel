
use std::fs;
use rusqlite::{Connection, Result};

#[path = "globals.rs"] mod globals;

use globals::Globals;

static DB_PATH: &str = "db.sqlite";



pub fn read() -> Vec<u8> {
    let contents = fs::read(DB_PATH)
        .expect("Something went wrong reading the file");
    //println!("{:?}", contents);
    return contents;
}

pub fn write(data: &String){
    if !std::path::Path::new(DB_PATH).exists() {
        return;
    }

    fs::write(DB_PATH, data).expect("Unable to write file");   
}

pub fn init() -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    println!("{:?}", conn);
    let globals = globals::Globals::new();
    Ok(())
}
