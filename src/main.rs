mod parser;
mod pipe;
mod misc;


use std::io;


fn main(){
    misc::throwitout();
    loop{
        misc::path();
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        parser::execute(&input);
        
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
    - built in function
    - tab autocomplete
    - uparrow
    https://docs.rs/crossterm/0.17.7/crossterm/event/struct.KeyEvent.html

    - fix error handling 
    https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html

    let err = exec::execvp("echo", &["echo", "foo"]);
    println!("Error: {}", err);

*/