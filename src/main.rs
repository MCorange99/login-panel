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



fn donothing(){
    // literaly do nothing lmao
}

fn com_user(usr: &String){
    print!("Current user: {}\n", usr.green());
}

fn com_login(user: String) -> String{

    print!("{}", "Username: ".green());  io::stdout().flush().unwrap();

    let usrnm: String = text_io::read!("{}\n");

    print!("{}", "Password: ".green()); io::stdout().flush().unwrap();

    let passwd: String = text_io::read!("{}\n");
    if format!("{usrnm}:{passwd}\n") == (db::get_by_id(usrnm.to_string())+ "\n"){
        println!("{}{}{}", "Logged in as ".yellow(), usrnm.green(), "!".yellow());
        return usrnm;
        
    } else {
        return user;
    }
}

fn main() {
    db::init();
    let _ = db::read();
    
    
    
    let mut user = "none".to_string();

    // user input loop
    loop {
        print!("{}@{}", user.green(), "panel>".blue()); io::stdout().flush().unwrap();

        let inp: String = text_io::read!("{}\n");

        match inp.as_str(){
            "help" => help(),
            "user" => com_user(&user.to_string()),
            "login" => user = com_login(user.to_string()),
            "exit" => return,
            "" => donothing(),
            _ => println!("{}{}{}", " - Command '".blue(), inp.red(), "' not found!".blue()),
        }
    }
}
