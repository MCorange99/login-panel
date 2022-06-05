use text_io;
use std::io::{self, Write};
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

fn main() {
    db::init();
    
    struct Data{
        user: String
    }
    impl Data{
     
    }
    
    let none: String = String::from("none");
    let data: Data = Data{user: none};

    // user input loop

    loop {
        print!("{}@{}", data.user.green(), "panel>".blue());
        io::stdout().flush().unwrap();
        let inp: String = text_io::read!("{}\n");

        match inp.as_str(){
            "help" => help(),
            "user" => com_user(&data.user),
            "" => donothing(),
            _ => println!("{}{}{}", " - Command '".blue(), inp.red(), "' not found!".blue()),
        }
    }
}
