use std::process;
struct Tree{
    name: char, 
    value: i32, 
    children: Vec<Tree>
}

pub fn create_struct( ) -> String{

    let output = process::Command::new("bash")
        .arg("-c")
        .arg("compgen -c")
        .stdout(process::Stdio::piped())
        .output()
        .expect("Error: compgen doesn't exist");
    let stdout = String::from_utf8(output.stdout).unwrap();
    let split: Vec<&str> = stdout.split("\n").collect();
    for i in split.into_iter(){
        println!("{}", i);
    }
    let tree = Tree;
    return stdout;

}



pub fn autocorrect(input: &str) {
    

}