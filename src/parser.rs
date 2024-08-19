
/*
Recall that we need to use the declarator "pub" to do something
*/

use std::process;
use std::env;
use std::path::Path;

use crate::misc::sighandler;

fn parse (input: &str)-> Vec<&str> {
    let mut tokens: Vec<&str> = Vec::new();

    let mut start = 0; 
    let mut end = 0; 
    // use a stack to figure out how to do the quotes 
    while end < input.len() {
        let c = input.chars().nth(end).unwrap();
        if c == ' ' || c == '\n' || c == '\t' {
            if end > start {
                tokens.push(&input[start..end]);
            }
            end += 1;
            start = end;
        }
        else {
            end += 1;
        }
    }

    // need to be able to parse the input differently
    // https://stackoverflow.com/questions/27475113/how-to-check-for-eof-with-read-line
    // we can use this one for EOF 

    return tokens;
}


pub fn execute (input: &str) {
    let sentence = input.trim().split(";"); // trim the carriage return / new line

    for token in sentence{
        let mut split = token.split_whitespace();
        let command = split.next().unwrap();

        // manage if command == cd / exit / 
        match command {
            "exit" => {
                process::exit(0);
            },
            "cd" => {
                let root: Vec<&str> = split.collect();
                if root.len() > 1{
                    println!("too many arguments"); 
                    // this isn't actually how cd works but for our purposes... let's pretend like it is
                }
                else {
                    let root = root.join("");
                    let path = Path::new(&root);
                    let success = env::set_current_dir(&path);
                        match success {
                            Ok(success) => {success},
                            Err(_error) => {
                                println!("cd: no such file or directory: {}", root);
                            }
                        }
                }
                
            },
            _ => { 
                // I should probably have a separate way to handle all of this
                let child = process::Command::new(command)
                .args(split)
                .spawn();
                
                match child {
                    Ok(mut child) => {
                        let childprocessid = child.id();
                        sighandler(childprocessid as i32); 
                        child.wait().expect("couldn't wait");
                    }
                    Err(_error) => {
                        println!("Command not found: {}", command);
                    }
                }

            
            }
        }

        
    }


}