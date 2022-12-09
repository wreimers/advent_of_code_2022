
use std::fs;
use super::lib::Puzzle;

#[allow(dead_code)]
pub fn day05_part1(data_file: &str) {
    let data: String = fs::read_to_string(data_file)
        .expect("Failed to read data_file");
    let halves = data.split("\n\n").collect::<Vec<&str>>();
    println!("halves:0\n{}", &halves[0]);
    println!("halves:1\n{}", &halves[1]);
    let puzzle_str = halves[0];
    let moves_str = halves[1];

    let mut puzzle = Puzzle::from_str(&puzzle_str);
    for moove in moves_str.split("\n") {
        println!("move:{}\n", moove);
    }


}

#[allow(dead_code)]
pub fn day05_part2(data_file: &str) {
    let data: String =fs::read_to_string(data_file)
        .expect("Should have been able to read the file");
    let mut total: u64 = 0;
    for line in data.split("\n") {
        total += 1;
        match line {
            "" => continue,
            _  => {},
        };
    }
    println!("lines: {}", total);
}
