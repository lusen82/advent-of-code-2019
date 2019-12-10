use std::str::FromStr;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

pub fn day_6() {
    let mut file = File::open("inp6.txt").unwrap();
    let reader = BufReader::new(file);
    let inp: Vec<String> = reader.lines().into_iter().map(|d| {
        let numbers = d.unwrap();
        numbers
    }).collect();
    let mut file = File::open("inp6.txt").unwrap();
    let reader = BufReader::new(file);
    let inp2: Vec<String> = reader.lines().into_iter().map(|d| {
        let numbers = d.unwrap();
        numbers
    }).collect();
    if let Ok(root_name) = String::from_str("COM") {
        let root: Node = Node{name: root_name, children: vec![]};
        let mut tree: HashMap<String, Vec<String>> = HashMap::new();
        inp.into_iter().for_each(|inprow|
            {
                let data: Vec<&str> = inprow.split(')').collect();
                tree.insert(data[0].to_string(),  vec![]);
            });

        inp2.into_iter().for_each(|inprow|
            {
                let data: Vec<&str> = inprow.split(')').collect();
                tree.get_mut(data[0]).unwrap().push(data[1].to_string());
            });

        tree.keys().into_iter().for_each(|key|
            println!("Printing key {} has values {:?}", key, tree.get(key).unwrap()));

        let sum = print_each_in_tree("COM".to_string(), &tree, 0);

        println!("Sum is {}", sum);
    }
}

fn print_each_in_tree(text: String, tree: &HashMap<String, Vec<String>>, number: i32) -> i32{
    let mut all = 0;
    let new_number = number + 1;
    if let Some(chilrd) = tree.get(text.as_str()) {
        for t in chilrd {
            let total = print_each_in_tree(t.clone(), tree, new_number);
            println!("Parent {} Child {} Number {} Text: {}", text, t.clone(), number, text);
            all += total;
        }
    }
    return number + all;
}

pub fn day_6_b() {
    let mut file = File::open("inp6.txt").unwrap();
    let reader = BufReader::new(file);
    let inp: Vec<String> = reader.lines().into_iter().map(|d| {
        let numbers = d.unwrap();
        numbers
    }).collect();
    let mut file = File::open("inp6.txt").unwrap();
    let reader = BufReader::new(file);
    let inp2: Vec<String> = reader.lines().into_iter().map(|d| {
        let numbers = d.unwrap();
        numbers
    }).collect();
    if let Ok(root_name) = String::from_str("COM") {
        let root: Node = Node{name: root_name, children: vec![]};
        let mut tree: HashMap<String, Vec<String>> = HashMap::new();
        inp.into_iter().for_each(|inprow|
            {
                let data: Vec<&str> = inprow.split(')').collect();
                tree.insert(data[1].to_string(),  vec![]);
            });

        inp2.into_iter().for_each(|inprow|
            {
                let data: Vec<&str> = inprow.split(')').collect();
                tree.get_mut(data[1]).unwrap().push(data[0].to_string());
            });

        tree.keys().into_iter().for_each(|key|
            println!("Printing key {} has values {:?}", key, tree.get(key).unwrap()));

        let you_res = get_parents("YOU".to_string(), &tree, vec![]);
        let san_res = get_parents("SAN".to_string(), &tree, vec![]);

        println!("you_res is {:?}", &you_res);
        println!("san_res is {:?}", &san_res);
        let mut i_you = -1;
        for value in &you_res {
            let mut j_you = -1;
            for other_branch in &san_res {
                if value == other_branch {
                    println!("Found a branch! i_you is {} j_you is {}, value is {}", i_you, j_you, value);
                    break;
                }
                j_you +=1;
            }
            i_you += 1;
        }
    }
}

#[derive(Clone)]
struct Node {
    name: String,
    children: Vec<Node>
}

fn get_parents(text: String, tree: &HashMap<String, Vec<String>>, parents: Vec<String>) -> Vec<String>  {
    let mut new_parents = parents.clone();
    new_parents.push(text.clone());
    if let Some(chilrd) = tree.get(text.as_str()) {
        return get_parents(chilrd.get(0).unwrap().clone(), tree, new_parents.clone());
    }
    return new_parents;
}

