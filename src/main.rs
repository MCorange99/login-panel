use text_io;
use std::{io::{self, Write}};
use colored::Colorize;

#[path = "db.rs"] mod db;
#[path = "commands.rs"] mod cmds;

fn help(){
    println!("Comands: ");
    println!("    help: Show this screen.");
    println!("    user: Print current logged in user.");
}

fn com_user(usr: &String){
    if usr == "none"{
        println!("{}", "Please login first!".red());
        return;
    }
    print!("Current user: {}\n", usr.green());
}

fn com_login(argv: Vec<&str>,user: String) -> String{

    if argv.len() < 2{
        println!("{}", "No user specified!".red());
        return user;
    }

    let usrnm: String = argv[1].to_string();

    print!("{}", "Password: ".green()); io::stdout().flush().unwrap();

    let usr: Vec<String> = db::get_by_id(&argv[1].to_string()).unwrap().unwrap().split(":").map(|s| s.to_string()).collect();
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

fn com_adduser(argv: Vec<&str>, user: String){
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

fn com_deluser(argv: Vec<&str>, user: String){
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

fn main() {
    db::init();
    let _ = db::read();
    
    
    
    let mut user = "none".to_string();

    // user input loop
    loop {
        print!("{}@{}", user.green(), "panel>".blue()); io::stdout().flush().unwrap();

        let inp: String = text_io::read!("{}\n");
        let argv: Vec<&str> = inp.split(" ").collect();
        //println!("{:?}", argv[1]);
        match argv[0]{
            "help" => help(),
            "user" => com_user(&user.to_string()),
            "userdel" => com_deluser(argv, user.to_string()),
            "useradd" => com_adduser(argv, user.to_string()),
            "login" => user = com_login(argv, user.to_string()),
            "exit" => return,
            "" => (),
            _ => println!("{}{}{}", " - Command '".blue(), inp.red(), "' not found!".blue()),
        }
    }
}
