use std::{fs, collections::{HashSet, HashMap}, char, ops::{RangeInclusive}};

#[allow(dead_code)]
pub fn day03(data_file: &str) {
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
