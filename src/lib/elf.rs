#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct Elf {
    food: Vec<u64>,
}

impl Default for Elf {
    fn default() -> Self {
        Self { food: Vec::new() }
    }
}

impl Elf {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn total_calories(&self) -> u64 {
        self.food.iter().sum()
    }

    pub fn food_count(&self) -> u64 {
        self.food.len().try_into().unwrap()
    }

    pub fn add_food(&mut self, calories: u64) {
        self.food.push(calories);
    }
}
