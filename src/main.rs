use std::{collections::HashMap, env::args};

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
    println!("day one part one")
}
