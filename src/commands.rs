use text_io;
use std::{io::{self, Write}};
use colored::Colorize;
#[path = "db.rs"] mod db;

pub fn user(usr: &String){
    if usr == "none"{
        println!("{}", "Please login first!".red());
        return;
    }
    print!("Current user: {}\n", usr.green());
}

pub fn login(argv: Vec<&str>,user: String) -> String{

    if argv.len() < 2{
        println!("{}", "No user specified!".red());
        return user;
    }

    let usrnm: String = argv[1].to_string();

    

    let tmp: Option<String> = db::get_by_id(&argv[1].to_string()).unwrap();
    if tmp == Option::None {
        println!("{}", "User does not exist!".red());
        return user;
    }
    let usr: Vec<String> = tmp.unwrap().split(":").map(|s| s.to_string()).collect();
    print!("{}", "Password: ".green()); io::stdout().flush().unwrap();
    // println!("asdasd: {:?}", usr);
    let _usr_nm = &usr[0];
    let usr_pwd = &usr[1];
    let _usr_admin = &usr[2];

    let passwd: String = text_io::read!("{}\n");
    if usr_pwd.to_string() == passwd.to_string(){
        println!("{}{}{}", "Logged in as ".yellow(), usrnm.green(), "!".yellow());
        return usrnm;
        
    } else {
        println!("{}", "Wrong password or username!".red());
        return user;
    }
}

pub fn adduser(argv: Vec<&str>, user: String){
    if user == "none"{
        println!("{}", "Please login first!".red());
        return;
    }
    if argv.len() < 2{
        println!("{}", "No user specified!".red());
        return ;
    }

    if db::get_by_id(&argv[1].to_string()).unwrap().unwrap() == ""{
        println!("{}", "User already exists!".red());
        return ;
    }

    let usrnm: String = argv[1].to_string();
    print!("{}", "Password: ".green()); io::stdout().flush().unwrap();
    let passwd: String = text_io::read!("{}\n");
    print!("{}", "Is Admin: ".green()); io::stdout().flush().unwrap();
    let is_op: String = text_io::read!("{}\n");

    let usr_cfg: Vec<String> = db::get_by_id(&user.to_string())
                                                            .unwrap()
                                                            .unwrap()
                                                            .split(":")
                                                            .map(|s| s.to_string())
                                                            .collect();

    if usr_cfg[2] != "true"{
        println!("{}", "You do not have permission to run this command!".red());
    }
    let line = format!("{usrnm}:{passwd}:{is_op}\n");
    let old = String::from_utf8_lossy(&db::read()).to_string();
    let full = old + "\n" + &line.to_string();
    db::write(&full.to_string());
}

pub fn deluser(argv: Vec<&str>, user: String){
    if user == "none"{
        println!("{}", "Please login first!".red());
        return;
    }
    if argv.len() < 2{
        println!("{}", "No user specified!".red());
        return ;
    }
    let usr_cfg: Vec<String> = db::get_by_id(&user).unwrap().unwrap().split(":").map(|s| s.to_string()).collect();
    let _cfg: Vec<String> = db::get_by_id(&argv[1].to_string()).unwrap().unwrap().split(":").map(|s| s.to_string()).collect();
    if usr_cfg[2] != "true"{
        println!("{}", "You do not have permission to run this command!".red());
        return;
    }
    db::delete_by_id(argv[1].to_string());
    println!("{}{}{}", "Succesfully deleted user ".yellow(), _cfg[0].green(), "!".yellow());
}

pub fn echo(mut data: Vec<&str>){
    data.remove(0);
    print!("{}\n", data.join(" "));
    return;
}

pub fn passwd(argv: Vec<&str>, user: &str){
    if user == "none"{
        println!("{}", "Please login first!".red());
        return;
    }
    
    print!("{}", "Enter Password: ".green()); io::stdout().flush().unwrap();
    let passwd: String = text_io::read!("{}\n");
    print!("{}", "Re Enter Password: ".green()); io::stdout().flush().unwrap();
    let passwd2: String = text_io::read!("{}\n");

    if passwd != passwd2 {
        println!("{}", "Passwords do not match!".red());
        return
    }
    if passwd == "" {
        println!("{}", "Password cant be blank!".red());
        return
    }
    //get db file
    let data = String::from_utf8_lossy(&db::read()).to_string();
    let mut all_users: Vec<&str> = data.split("\n").collect();
    let mut i = 0;
    //loop thru users till u find one
    for u in data.split("\n") {
        if u == "" {
            continue;
        }
        let mut user_data: Vec<&str> = u.split(":").collect();
        if argv.len() < 2 {
            let usrnm = user;
            if user_data[0] == usrnm {
                // change the password
                user_data[1] = &passwd;
                let temp = user_data.join(":");
                all_users[i] = temp.as_str();
                // write the changes to db
                db::write(&all_users.join("\n"));
                println!("{}{}{}", "Succesfully changed the password for user ".yellow(), usrnm.green(), "!".yellow());
                return
            }
        } else{
            let usrnm: &str = argv[1];
            if user_data[0] == usrnm {
                // change the password
                user_data[1] = &passwd;
                let temp = user_data.join(":");
                all_users[i] = temp.as_str();
                // write the changes to db
                db::write(&all_users.join("\n"));
                println!("{}{}{}", "Succesfully changed the password for user ".yellow(), usrnm.green(), "!".yellow());
                return
            }
        }
        
        i = i + 1;
        
    }
    
    // only reachable of user doesnt exist
    println!("{}", "User does not exist!".red());
    
    
}