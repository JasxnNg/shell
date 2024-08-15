use whoami;
use std::env::current_dir;
use std::io::{self, Write};
use colored::Colorize;

pub fn path(){
    // username, device name, and path name
    let user: String = whoami::username();
    let device: String = whoami::fallible::hostname().unwrap();
    let path = current_dir().expect("Couldn't obtain the current path");
    
    // I can't fuck with the borrow checker
    let path_str = path.to_str().expect("Couldn't convert path to an UTF-8 String");
    // Split the path by the username and collect into a Vec<&str>
    let parts: Vec<&str> = path_str.split(&user).collect();

    if parts.len() > 1 {
        let tilde = "~";
        let joined_path = &(format!("{}{}", tilde, parts[1]));
        print!("{}{}{}:{}$ ", user.green(), "@".purple(), device.green(), joined_path.purple());
    }
    else {
        let joined_path = parts[0];
        print!("{}{}{}:{}$ ", user.green(), "@".purple(), device.green(), joined_path.purple());
    }


    io::stdout().flush().unwrap();

}