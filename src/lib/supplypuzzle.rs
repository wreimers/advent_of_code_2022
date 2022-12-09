// lib/supplycpuzzle.rs

use super::supplystack::Stack;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Puzzle {
    stacks: Vec<Stack>,
}

impl Puzzle {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Default::default()
    }
    pub fn from_str(strn: &str) -> Self {
        let mut lines: Vec<&str> = strn.split("\n").collect();
        // read the last line!
        let last_line_no = lines.len() - 1;
        let last_line = lines[last_line_no];
        println!("last_line {}", last_line);
        Self { stacks: Vec::new() }
    }
}

impl Default for Puzzle {
    fn default() -> Self {
        Self { stacks: Vec::new() }
    }
}
