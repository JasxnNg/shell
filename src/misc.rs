use whoami;
use std::env::current_dir;
use std::io::{self, Write};
use colored::Colorize;



// unistd is just a glibc wrapper LOL 
use nix::sys::signal;
use nix::sys::signal::Signal;
use nix::unistd::Pid;
use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;
use std::thread;



pub fn sighandler (childprocessid: i32)  {
    // handle sigint 
    let mut signals = Signals::new(&[SIGINT, SIGTERM, SIGHUP]).expect("Failed to create signal handler");

    // Spawn a thread to handle the signals
    thread::spawn(move || {
        for sig in signals.forever() {
            match sig {
                SIGINT => {
                    // println!("Received SIGINT, killing child process...");
                    let var = signal::kill(Pid::from_raw(childprocessid), Signal::SIGTERM);
                    match var {
                        Ok(_) => {
                            
                        
                        }
                        Err(_) => {
                            
                        }
                    }
                }
                SIGTERM => {
                    println!("Received SIGTERM, exiting...");
                    std::process::exit(0);
                }
                SIGHUP => {
                    println!("Received SIGHUP, reloading configuration...");
                    // Add your configuration reload logic here
                }
                _ => unreachable!(),
            }
        }
    });

}


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