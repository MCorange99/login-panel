use std::{fs::File};
use std::io::Write;
use std::fs;

use std::fs::OpenOptions;
#[allow(unused_imports)]
use std::io::{prelude::*, Seek, SeekFrom};

#[allow(dead_code)]
pub fn read() -> Vec<u8> {
    let contents = fs::read("creds.db")
        .expect("Something went wrong reading the file");
    //println!("{:?}", contents);
    return contents;
}

#[allow(dead_code)]
fn write(data: &String){
    if !std::path::Path::new("creds.db").exists() {
        return;
    }

    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("creds.db")
        .unwrap();

    file.seek(SeekFrom::Start(0)).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

fn init_new(admin_usrnm: &str, admin_passwd: &str) -> std::io::Result<()> {
    let mut file = File::create("creds.db")?;
    let data = format!("{admin_usrnm}:{admin_passwd}\n");
    file.write_all(data.as_bytes())?;
    println!("Initialiased a new database with a root account. The default password is 'toor'");
    Ok(())
}

pub fn init(){
    if !std::path::Path::new("creds.db").exists() {
        let _ = init_new("root", "toor");
    }
}

#[allow(dead_code)]
pub fn get_by_id(id: String) -> String {
    let data = String::from_utf8_lossy(&read()).to_string();
    let lines = data.lines();
    //println!("{:?}", lines);
    for user in lines {
        let idk: Vec<String> = user.split(":").map(|s| s.to_string()).collect();
        if idk[0] == id {
            return user.to_string();
        }

    }
    
    return "".to_string();
}

#[allow(dead_code)]
pub fn save(){

}