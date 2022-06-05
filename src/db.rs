use std::fs::File;
use std::io::Write;

#[allow(dead_code)]
fn read(){
    if !std::path::Path::new("creds.db").exists() {
        return;
    }
}

#[allow(dead_code)]
fn write(){
    if !std::path::Path::new("creds.db").exists() {
        return;
    }
}

fn init_new(admin_usrnm: &str, admin_passwd: &str) -> std::io::Result<()> {
    let mut file = File::create("creds.db")?;
    let data = String::from(admin_usrnm.to_owned() + ":" + admin_passwd + "\n"); 
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
pub fn get_by_id(){

}

#[allow(dead_code)]
pub fn save(){

}