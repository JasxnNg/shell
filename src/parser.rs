
/*
Recall that we need to use the declarator "pub" to do something
*/
use std::process;
use std::env;
use std::path::Path;

// fn parse(input: SplitWhitespace) {

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
                let mut child = process::Command::new(command)
                .args(split)
                .spawn()
                .expect("command failed");

                child.wait().expect("couldn't wait");
            }
        }

        
    }


}