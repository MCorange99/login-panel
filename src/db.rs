use std::{fs::File};
use std::io::{Write, BufReader, self};
use std::fs;

#[allow(unused_imports)]
use std::io::{prelude::*, Seek, SeekFrom};


static DB_PATH: &str = "creds.db";

// these are all used it just doesnt show for some reason
pub fn read() -> Vec<u8> {
    let contents = fs::read(DB_PATH)
        .expect("Something went wrong reading the file");
    //println!("{:?}", contents);
    return contents;
}
#[allow(dead_code)]
pub fn write(data: &String){
    if !std::path::Path::new(DB_PATH).exists() {
        return;
    }

    fs::write(DB_PATH, data).expect("Unable to write file");   
}
#[allow(dead_code)]
fn init_new(admin_usrnm: &str, admin_passwd: &str) -> std::io::Result<()> {
    let mut file = File::create(DB_PATH)?;
    let data = format!("{admin_usrnm}:{admin_passwd}:true\n");
    file.write_all(data.as_bytes())?;
    println!("Initialiased a new database with a root account. The default password is 'toor'");
    Ok(())
}

#[allow(dead_code)]
pub fn init(){
    if !std::path::Path::new(DB_PATH).exists() {
        let _ = init_new("root", "toor");
    }
}


#[allow(dead_code)]
pub fn get_by_id(id: &str) -> io::Result<Option<String>> {
    let rdr = BufReader::new(File::open(DB_PATH)?);
    for user in rdr.lines() {
        let user = user?;
        if let Some((uid, _)) = user.split_once(':') {
            if uid == id {
                return Ok(Some(user.to_string()));
            }
        }
    }
    Ok(None)
}
#[allow(dead_code)]
pub fn delete_by_id(id: String){
    let data = String::from_utf8_lossy(&read()).to_string();
    let lines = data.lines();
    //println!("{:?}", lines);
    let mut modified = Vec::new();
    for user in lines {
        let idk: Vec<String> = user.split(":").map(|s| s.to_string()).collect();
        if idk[0] != id {
            modified.push(user.to_string());
        }
    
    }
    // println!("{:?}", modified);
    write(&modified.join("\n"))
}
