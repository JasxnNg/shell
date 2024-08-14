use whoami;
use std::env::current_dir;
use std::io::{self, Write};
use colored::Colorize;

pub fn path(){
    let user = whoami::username();
    let device = whoami::fallible::hostname().unwrap();
    let path = current_dir().expect("Couldn't obtain the current path");
    let str = path.to_str().expect("Couldn't convert path to an UTF-8 String");

    print!("{}@{}:{}$ ", user.green(), device.green(), str.purple());
    io::stdout().flush().unwrap();

}