
/*
Recall that we need to use the declarator "pub" to do something
*/
use std::process;
use std::env;
use std::path::Path;

// fn parse(input: SplitWhitespace) {

    // need to be able to parse the input differently


// }


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
                    println!("too many arguments")
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
                let child = process::Command::new(command)
                .args(split)
                .spawn();
                
                match child {
                    Ok(mut child) => {child.wait().expect("couldn't wait");}
                    Err(_error) => {
                        println!("Command not found: {}", command);
                    }
                }

            
            }
        }

        
    }


}