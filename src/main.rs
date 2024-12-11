use std::{collections::HashMap, env::args, fs};

fn main() {
    // let mut input = args();
    println!("Advent of Code 2024!");
    let arg = args().nth(1).unwrap();
    println!("input argument: {:?}", &arg);
    let mut map: HashMap<String, fn()> = HashMap::new();
    map.insert("d1p1".to_string(), day_one_part_one);
    map.get(&arg).unwrap()();
}

fn day_one_part_one() {
    println!("day one part one");
    let data = retrieve_input_data("./puzzle_input/d1p1.txt");
    let col1 = Vec::new();
    let col2 = Vec::new();
    for line in data.split("\n") {
        let (first, second): (String, String) = line.split(" ").collect();
        col1.push(first.parse::<i32>().unwrap());
        col2.push(second.parse::<i32>().unwrap());
    }
    col1.sort();
    col2.sort();
    println!("column1: {}", col1);
    println!("column2: {}", col2);
}

fn retrieve_input_data(path: str) -> String {
    fs::read_to_string(path).unwrap()
}
