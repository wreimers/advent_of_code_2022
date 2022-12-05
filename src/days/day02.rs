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
            "A X" => score += ROCK     + DRAW, // Rock     v. Rock
            "A Y" => score += PAPER    + WIN,  // Rock     v. Paper
            "A Z" => score += SCISSORS + LOSE, // Rock     v. Scissors
            "B X" => score += ROCK     + LOSE, // Paper    v. Rock
            "B Y" => score += PAPER    + DRAW, // Paper    v. Paper
            "B Z" => score += SCISSORS + WIN,  // Paper    v. Scissors
            "C X" => score += ROCK     + WIN,  // Scissors v. Rock
            "C Y" => score += PAPER    + LOSE, // Scissors v. Paper
            "C Z" => score += SCISSORS + DRAW, // Scissors v. Scissors
            _     => panic!("Invalid token in data file"),
        };
    }
    println!("Score: {}", score)
}
