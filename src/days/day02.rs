use std::fs;

const ROCK: u64     = 1;
const PAPER: u64    = 2;
const SCISSORS: u64 = 3;

const WIN: u64      = 6;
const LOSE: u64     = 0;
const DRAW: u64     = 3;

#[allow(dead_code)]
pub fn day02(data_file: &str) {
    let data: String =fs::read_to_string(data_file)
        .expect("Should have been able to read the file");
    let mut score: u64 = 0;
    for line in data.split("\n") {
        match line {
            ""    => continue,
            "A X" => score += SCISSORS + LOSE,
            "A Y" => score += ROCK     + DRAW,
            "A Z" => score += PAPER    + WIN,
            "B X" => score += ROCK     + LOSE,
            "B Y" => score += PAPER    + DRAW,
            "B Z" => score += SCISSORS + WIN,
            "C X" => score += PAPER    + LOSE,
            "C Y" => score += SCISSORS + DRAW,
            "C Z" => score += ROCK     + WIN,
            _     => panic!("Invalid token in data file"),
        };
    }
    println!("Score: {}", score)
}
