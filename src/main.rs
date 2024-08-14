mod parser;
mod pipe;
mod misc;

use std::process::Command;
use std::io;


fn main(){
    loop{
    misc::path();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

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
    - error handling 

    let err = exec::execvp("echo", &["echo", "foo"]);
    println!("Error: {}", err);

*/