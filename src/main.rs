mod parser;
mod pipe;

use std::process::Command;
use std::io::stdin;
use std::env;
use whoami;
use colored::Colorize;

fn path(){
    let user = whoami::username();
    let device = whoami::devicename();
    let path = env::current_dir().expect("what the sigma");
    print!("{}@{}: {}", user, device, path.display()).flush();

}
fn main(){
    loop{
    path();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut split = input.trim().split_whitespace();
    let command = split.next().unwrap();
    let args = split;

    // read_line leaves a trailing newline, which trim removes
    // let command = input.trim(); 

    // println!("Running command: {}", command);

    Command::new(command)
        .args(args)
        .spawn()
        .unwrap();
    }
}
/*

Note to self: 
    - get documentation of execvp
    - read the purdue thing of execvp 
    - read fork documentation 
    - lexer and pipe 
    - strtokenizer

    let err = exec::execvp("echo", &["echo", "foo"]);
    println!("Error: {}", err);

*/