use std::{collections::HashMap, env::args, fs, ops::IndexMut};

fn main() {
    println!("Advent of Code 2024!");
    let arg = args().nth(1).unwrap();
    println!("input argument: {:?}", &arg);
    let mut map: HashMap<String, fn()> = HashMap::new();
    map.insert("d1p1".to_string(), day_one_part_one);
    map.insert("d1p2".to_string(), day_one_part_two);
    map.get(&arg).unwrap()();
}

fn day_one_part_one() {
    println!("day one part one");
    let data = retrieve_input_data("./puzzle_input/d1p1.txt");
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();
    for line in data.split("\n") {
        if line == "" {break;}
        let mut iter = line.split_whitespace();
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        col1.push(first.parse::<i32>().unwrap());
        col2.push(second.parse::<i32>().unwrap());
    }
    col1.sort();
    col2.sort();

    let mut sum = 0;
    for (idx, id) in col1.iter().enumerate() {
        sum += i32::abs(id - *col2.index_mut(idx));
    }
    println!("total summed distance between ids: {}", sum)
}

fn retrieve_input_data(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn day_one_part_two() {
    println!("day one part two");
    let data = retrieve_input_data("./puzzle_input/d1p1.txt");
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();
    for line in data.split("\n") {
        if line == "" {break;}
        let mut iter = line.split_whitespace();
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        col1.push(first.parse::<i32>().unwrap());
        col2.push(second.parse::<i32>().unwrap());
    }
    col1.sort();
    col2.sort();
    let mut id_map = HashMap::new();
    for i in col2 {
        id_map.entry(i).and_modify(|v| *v += 1).or_insert(1);
    }
    let mut sum = 0;
    for i in col1 {
        if let Some(num) = id_map.get(&i) {
            sum += i*num;
        }
    }
    println!("multiplied and summed numbers from list one that were in list two: {}", sum)
}
