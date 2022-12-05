use std::fs;

#[derive(Debug, Clone)]
struct Elf {
    food: Vec<u64>,
}

impl Elf {
    fn total_calories(&self) -> u64 {
        self.food.iter().sum()
    }

    fn food_count(&self) -> u64 {
        self.food.len().try_into().unwrap()
    }

    fn add_food(&mut self, calories: u64) {
        self.food.push(calories);
    }
}

fn parse_data_file() -> Vec<Elf> {
    // let data_file: &str = "day01_testcase.txt";
    let data_file: &str = "day01_star01.txt";
    let mut elves: Vec<Elf>  = Vec::new();
    let mut high_score_0: u64 = 0;
    let mut high_score_1: u64 = 0;
    let mut high_score_2: u64 = 0;
    let data: String =fs::read_to_string(data_file)
        .expect("Should have been able to read the file");
    let mut elf: Elf = Elf { food: Vec::new() };
    for line in data.split("\n") {
        if line == "" {
            if elf.food_count() > 0 {
                // println!("This elf is carrying {} food items worth {} calories.", elf.food_count(), elf.total_calories());
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
            elf = Elf { food: Vec::new() };
            continue;
        }
        elf.add_food(line.trim().parse().expect("Wanted a number"));
        // println!("{}", line);
    }
    println!("High scores: {} {} {}", high_score_0, high_score_1, high_score_2);
    println!("Sum: {}", high_score_0 +high_score_1 + high_score_2);

    elves
}

fn main() {
    let _elves = parse_data_file();
}
