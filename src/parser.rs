
/*
Recall that we need to use the declarator "pub" to do something
*/

use std::process;
use std::env;
use std::path::Path;

use nix::libc::boolean_t;

use crate::autocomplete::Tree;
use crate::autocomplete;
use crate::misc;


fn parse (input: &str)-> Vec<&str> {
    let mut tokens: Vec<&str> = Vec::new();

    let mut start = 0; 
    let mut end = 0; 

    // let mut char = 0;
    // input.chars()
    //     .for_each(|c|  {
    //         print!("{}: {}", char, c);
    //         char+=1;
    //          // this is a Result and should be handled properly
    //     });
    // println!();
    // println!("hi : {} {}", input.chars().nth(end).unwrap(), input.len());
    // interesting behavior
/*
jasxnng@jing:~/Work/shell$ ∂
∂
0: ∂
hi : ∂ 3
thread 'main' panicked at src/parser.rs:30:40:
called `Option::unwrap()` on a `None` value
//https://stackoverflow.com/questions/23430735/how-to-convert-vecchar-to-a-string
 */
    while end < input.len() {
        // this doesn't really work for non-ascii characters 
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
    tokens.push(&input[start..end]);

    // need to be able to parse the input differently
    // https://stackoverflow.com/questions/27475113/how-to-check-for-eof-with-read-line
    // we can use this one for EOF 

    return tokens;
}


pub fn execute (input: &str, autocomplete: &mut Tree) {
    
    // there's other interesting behavior when it comes to shells with EOF
    if input.len() == 0{
        println!("\nDetected an EOF character. Exiting process...");
        process::exit(0);
    }
        
    let sentence = input.trim().split(";"); // trim the carriage return / new line
    
    // I should add a method to just print out a new thing if we have empty
    for token in sentence{
        
        let mut parsedcommand = parse(token);
        let command = parsedcommand[0]; 
        parsedcommand.remove(0);
        let split = parsedcommand;
        // remove_first(&mut parsedcommand);
        // let split = parsedcommand;
        // manage if command == cd / exit / 
        match command {
            "exit" => {
                process::exit(0);
            },
            "cd" => {
                
        
                if split.len() > 1{
                    println!("too many arguments"); 
                    // this isn't actually how cd works but for our purposes... let's pretend like it is
                    
                }
                else {
                    let root = split.join("");

                    if root.len() == 0 || root == "~" {
                        let user = misc::get_user();
                        let pathname = format!("/Users/{}", user);
                        let path = Path::new(&pathname);
                        let success = env::set_current_dir(&path);
                        match success {
                            Ok(success) => {success},
                            Err(_error) => {
                                println!("cd: no such file or directory: {}", root);
                            }
                        }
                    }
                    else {
                        let path = Path::new(&root);
                    
                        let success = env::set_current_dir(&path);
                            match success {
                                Ok(success) => {success},
                                Err(_error) => {
                                    println!("cd: no such file or directory: {}", root);
                                }
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
                        misc::sighandler(childprocessid as i32); 
                        child.wait().expect("couldn't wait");
                    }
                    Err(_error) => {
                        println!("Command not found: {}", command);
                        if command == "" {
                            println!("Please enter a command");
                        }
                        else {
                            let mut boolean = true;
                            let mut copy: &mut Tree = autocomplete;
                            for val in command.chars() {
                                let string = val.to_string();
                                if copy.children.contains_key(&string) {
                                    copy = copy.children.get_mut(&string).unwrap();
                                } else {
                                    boolean = false;
                                    break;
                                }
                                // println!("{}", val);
                            }
                            if boolean {
                                let val = autocomplete::create_all_variables(&command[..command.len() - 1], copy);
                                for variable in val {
                                    println!("{}", variable);
                                }
                            }

                    }
                }

            
            }
        }

        
    }


}

}

