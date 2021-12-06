use crate::command::Command;
use crate::operation::Operation;

#[derive(Debug, PartialEq)]
pub struct SubmarinePosition {
    pub depth: u32,
    pub horizontal: u32,
    pub aim: u32,
}

impl SubmarinePosition {
    pub fn new() -> Self {
        SubmarinePosition {
            depth: 0,
            horizontal: 0,
            aim: 0,
        }
    }

    pub fn process_command(&mut self, c: &Command) {
        let (op, n) = (c.0, c.1);
        match op {
            Operation::FORWARD => self.horizontal += n,
            Operation::UP => self.depth -= n,
            Operation::DOWN => self.depth += n,
        }
    }

    pub fn process_new_command(&mut self, c: &Command) {
        let (op, n) = (c.0, c.1);
        match op {
            Operation::FORWARD => {
                self.horizontal += n;
                self.depth += self.aim * n;
            }
            Operation::UP => self.aim -= n,
            Operation::DOWN => self.aim += n,
        }
    }
}