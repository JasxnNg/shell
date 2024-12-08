use std::{collections::HashMap, process};
pub struct Tree<'a>{
    pub name: &'a str, 
    pub value: i32, 
    pub children: HashMap<String, Tree<'a>>
}


pub fn create_struct( ) -> Tree<'static>{

    let output = process::Command::new("bash")
        .arg("-c")
        .arg("compgen -c")
        .stdout(process::Stdio::piped())
        .output()
        .expect("Error: compgen doesn't exist");
    let stdout = String::from_utf8(output.stdout).unwrap();
    let split: Vec<&str> = stdout.split("\n").collect();
    let mut tree: Tree = Tree {
        name: "",
        value: 0, 
        children: HashMap::new()
    };

    for string in split.into_iter() {
        let mut copy = &mut tree;
        for val in string.chars() {
            let string = val.to_string();
            if copy.children.contains_key(&string) {
                copy = copy.children.get_mut(&string).unwrap();
            } else {
                let new_tree = Tree {
                    name: Box::leak(string.clone().into_boxed_str()),
                    value: 0,
                    children: HashMap::new(),
                };
                copy.children.insert(string.clone(), new_tree);
                copy = copy.children.get_mut(&string).unwrap();
            }
        }
        copy.value += 1; // change the final copy value and increase it by 1
    }

    return tree;

}

pub fn create_all_variables(input: &str, tree : &Tree) -> Vec<String> {
    let mut variables: Vec<String> = Vec::new();
    let mut stack: Vec<(&Tree, String)> = Vec::new();
    stack.push((tree, "".to_string()));
    while !stack.is_empty() {
        let (node, mut string) = stack.pop().unwrap();
        string.push_str(node.name);
        if node.value > 0 {
            variables.push(input.to_owned() + &string.clone());
        }
        for (_, value) in node.children.iter() {
            stack.push((value, string.clone()));
        }
        if variables.len() > 10 {
            break;
        }
    }

    return variables;
}
