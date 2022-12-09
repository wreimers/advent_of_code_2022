// lib/supplycstack.rs

use super::supplycrate::Crate;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Stack {
    crates: Vec<Crate>,
}

#[allow(dead_code)]
impl Stack {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self { crates: Vec::new() }
    }
}
