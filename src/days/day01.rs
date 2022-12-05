use std::fs;
use crate::elf::Elf;

#[allow(dead_code)]
pub fn day01(data_file: &str) {
    let mut elves: Vec<Elf>  = Vec::new();
    let mut high_score_0: u64 = 0;
    let mut high_score_1: u64 = 0;
    let mut high_score_2: u64 = 0;
    let data: String =fs::read_to_string(data_file)
        .expect("Should have been able to read the file");
    let mut elf: Elf = Elf::new();
    for line in data.split("\n") {
        if line == "" {
            if elf.food_count() > 0 {
                if elf.total_calories() > high_score_0 {
                    high_score_2 = high_score_1;
                    high_score_1 = high_score_0;
                    high_score_0 = elf.total_calories();
                }
                else if elf.total_calories() > high_score_1 {
                    high_score_2 = high_score_1;
                    high_score_1 = elf.total_calories();
                }
                else if elf.total_calories() > high_score_2 {
                    high_score_2 = elf.total_calories();
                }
                elves.push(elf.clone());
            }
            elf = Elf::new();
            continue;
        }
        elf.add_food(line.trim().parse().expect("Wanted a number"));
    }
    println!("High scores: {} {} {}", high_score_0, high_score_1, high_score_2);
    println!("Sum: {}", high_score_0 +high_score_1 + high_score_2);
}
