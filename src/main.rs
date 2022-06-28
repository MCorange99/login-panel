use text_io;
use std::{io::{self, Write}};
use colored::Colorize;

#[path = "db.rs"] mod db;
#[path = "commands.rs"] mod cmds;

fn help(){
    println!("Comands: ");
    println!("       help: Show this screen.");
    println!("    userdel: Remove an user.");
    println!("    useradd: Add an user");
    println!("      login: Login to an user account.");
    println!("       exit: exit the program");
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
            "user" => cmds::user(&user.to_string()),
            "userdel" => cmds::deluser(argv, user.to_string()),
            "useradd" => cmds::adduser(argv, user.to_string()),
            "login" => user = cmds::login(argv, user.to_string()),
            "echo" => cmds::echo(argv),
            "passwd" => cmds::passwd(argv, &user),
            "exit" => return,
            "" => (),
            _ => println!("{}{}{}", " - Command '".blue(), inp.red(), "' not found!".blue()),
        }
    }
}
