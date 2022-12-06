use std::{fs, ops::{RangeInclusive}, collections::HashSet};

#[allow(dead_code)]
pub fn day04_part1(data_file: &str) {
    let data: String =fs::read_to_string(data_file)
        .expect("Should have been able to read the file");
    let mut total: u64 = 0;
    for line in data.split("\n") {
        match line {
            "" => continue,
            _  => {},
        };
        let ranges: Vec<&str> = line.split(",").collect();
        let first_bounds: Vec<u64>   = ranges[0].split("-").map(|n| n.trim().parse::<u64>().unwrap()).collect();
        let second_bounds: Vec<u64>  = ranges[1].split("-").map(|n| n.trim().parse::<u64>().unwrap()).collect();
        let first_range: HashSet<u64> = range_to_set(first_bounds[0]..=first_bounds[1]);
        let second_range: HashSet<u64> = range_to_set(second_bounds[0]..=second_bounds[1]);
        if first_range.is_subset(&second_range) {
            total += 1;
        }
        else if second_range.is_subset(&first_range) {
            total += 1;
        }
    }
    println!("contained pairs: {}", total);
}

#[allow(dead_code)]
pub fn day04_part2(data_file: &str) {
    let data: String =fs::read_to_string(data_file)
        .expect("Should have been able to read the file");
    let mut total: u64 = 0;
    for line in data.split("\n") {
        match line {
            "" => continue,
            _  => {},
        };
        let (first_range, second_range): (HashSet<u64>, HashSet<u64>) = parse_ranges_from_line(line);
        if ! first_range.is_disjoint(&second_range) {
            total += 1;
        }
    }
    println!("overlapping pairs: {}", total);  
}

fn parse_ranges_from_line(line: &str) -> (HashSet<u64>, HashSet<u64>) {
    let ranges: Vec<&str> = line.split(",").collect();
    let first_bounds: Vec<u64>   = ranges[0].split("-").map(|n| n.trim().parse::<u64>().unwrap()).collect();
    let second_bounds: Vec<u64>  = ranges[1].split("-").map(|n| n.trim().parse::<u64>().unwrap()).collect();
    let first_range: HashSet<u64> = range_to_set(first_bounds[0]..=first_bounds[1]);
    let second_range: HashSet<u64> = range_to_set(second_bounds[0]..=second_bounds[1]);

    (first_range, second_range)
}

fn range_to_set(range: RangeInclusive<u64>) -> HashSet<u64> {
    let mut set: HashSet<u64> = HashSet::new();
    if range.start() == range.end() {
        set.insert(*range.start());
    }
    else {
        range.clone().for_each(|number| {
            set.insert(number);
        });
    }

    set
}
