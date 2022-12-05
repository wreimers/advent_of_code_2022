use std::{fs, collections::{HashSet, HashMap}, char, ops::{RangeInclusive}};

#[allow(dead_code)]
pub fn day03_part2(data_file: &str) {
    let data: String =fs::read_to_string(data_file)
        .expect("Should have been able to read the file");
    let priorities = generate_priority_map();
    let mut total: u64 = 0;
    let mut data_iter = data.split("\n").peekable();
    loop {
        if data_iter.peek().is_none() {
            break;
        }
        if data_iter.peek().unwrap() == &"" {
            data_iter.next();
            continue;
        }
        let elf0 = slice_to_set(data_iter.next().unwrap());
        let elf1 = slice_to_set(data_iter.next().unwrap());
        let elf2 = slice_to_set(data_iter.next().unwrap());
        let intersection0: HashSet<char> = elf0.intersection(&elf1).copied().collect();
        let intersection1: HashSet<char> = elf2.intersection(&intersection0).copied().collect();
        let item_type = intersection1.iter().next().unwrap();
        total += priorities[item_type];
        println!("item_type {}", item_type);
    }
    println!("Total: {}", total);
}

#[allow(dead_code)]
pub fn day03_part1(data_file: &str) {
    let data: String =fs::read_to_string(data_file)
        .expect("Should have been able to read the file");
    let mut characters: Vec<char> = Vec::new();
    for line in data.split("\n") {
        match line {
            "" => continue,
            _  => {},
        };
        characters.push(find_item_type(line));
    }
    let priorities = generate_priority_map();
    let mut total: u64 = 0;
    for item_type in characters {
        print!("{} -> ", item_type);
        let value = priorities[&item_type];
        println!("{}", value);
        total += value;
    }
    println!("Total: {}", total);
}

fn generate_priority_map() -> HashMap<char, u64> {
    let keys = ('a'..='z').chain('A'..='Z');
    let values: RangeInclusive<u64> = 1..=52;
    HashMap::from(
        keys.into_iter()
        .zip(values.into_iter())
        .collect::<HashMap<char, u64>>()
    )
}

fn split_contents(contents: &str) -> (&str, &str) {
    let first_contents  = &contents[..((contents.len()/2))];
    let second_contents = &contents[((contents.len()/2))..];

    (first_contents, second_contents)
}

fn slice_to_set(slice: &str) -> HashSet<char> {
    let mut set = HashSet::<char>::new();
    for character in slice.chars() {
        set.insert(character);
    }

    set
}

fn find_item_type(contents: &str) -> char {
    let (first_contents, second_contents) = split_contents(contents);
    let first_set = slice_to_set(&first_contents);
    let second_set = slice_to_set(&second_contents);
    let item_type = first_set.intersection(&second_set).next().unwrap();

    item_type.clone()
}
