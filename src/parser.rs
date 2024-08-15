/*
Recall that we need to use the declarator "pub" to do something
*/
use std::process::Command;

fn parse() {

}


pub fn execute (input: &str) {
    let sentence = input.trim().split(";"); // trim the carriage return / new line

    for token in sentence{
        let mut split = token.split_whitespace();
        let command = split.next().unwrap();

        let mut child = Command::new(command)
            .args(split)
            .spawn()
            .expect("command failed");
        child.wait().expect("couldn't wait");
    }


}