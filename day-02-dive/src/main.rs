#![allow(dead_code)]

use std::{convert::{TryFrom, TryInto}, error::Error};

#[derive(Debug)]
pub struct Command<'a> (&'a str, u32);

impl<'a> TryFrom<&'a str> for Command<'a> {
    type Error = Box<dyn Error>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        // this all seems very clumsy
        let split_string = s.split(' ').collect::<Vec<_>>();
        match split_string.as_slice() {
            [op, n] if n.parse::<u32>().is_ok() => Ok(Command(op.clone(), n.parse()?)),
            _ => Err("Malformed command".into()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SubmarinePosition {
    depth: u32,
    horizontal: u32,
    aim: u32,
}

impl SubmarinePosition {
    pub fn new() -> Self {
        SubmarinePosition {
            depth: 0,
            horizontal: 0,
            aim: 0,
        }
    }

    pub fn process_command(&mut self, c: &Command) -> Result<(), Box<dyn Error>> {
        let (op, n) = (c.0, c.1);
        match op {
            "forward" => self.horizontal += n,
            "up" => self.depth -= n,
            "down" => self.depth += n,
            _ => return Err("Unknown command".into()),
        }
        Ok(())
    }

    pub fn process_new_command(&mut self, c: &Command) -> Result<(), Box<dyn Error>> {
        let (op, n) = (c.0, c.1);
        match op {
            "forward" => {
                self.horizontal += n;
                self.depth += self.aim * n;
            }
            "up" => self.aim -= n,
            "down" => self.aim += n,
            _ => return Err("Unknown command".into()),
        }
        Ok(())
    }
}

pub fn run_sub_directions(in_str: &str) -> SubmarinePosition {
    let vec_commands = in_str
        .lines()
        .filter_map(|line| Command::try_from(line).ok()) // look! Both TryFrom .. (see ln 79)
        .collect::<Vec<Command>>();
    let mut sub = SubmarinePosition::new();
    for cmd in vec_commands {
        sub.process_command(&cmd).unwrap();
    }
    sub
}

pub fn run_sub_with_new_directions(in_str: &str) -> SubmarinePosition {
    let vec_commands = in_str
        .lines()
        // .filter_map(|line| Command::try_from(line).ok())
        .filter_map(|line| line.try_into().ok()) // and TryInto! (from ln 66)
        .collect::<Vec<Command>>();
    let mut sub = SubmarinePosition::new();
    for cmd in vec_commands {
        sub.process_new_command(&cmd).unwrap();
    }
    sub
}

pub const INPUT: &str = include_str!("../input.txt");

fn main() {
    // let sub = run_sub_directions(INPUT);
    // println!("final position: {:?}", sub);

    let sub = run_sub_with_new_directions(INPUT);
    println!("final position: {:?}", sub);

    println!("multiplied: {}", sub.depth * sub.horizontal);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_runs_the_sim() {
        let sub = run_sub_directions(INPUT);
        assert_eq!(sub.depth, 907);
        assert_eq!(sub.horizontal, 1905);
    }

    #[test]
    fn it_runs_the_second_sim() {
        let sub = run_sub_with_new_directions(INPUT);
        assert_eq!(sub.depth, 810499);
        assert_eq!(sub.horizontal, 1905);
        assert_eq!(sub.aim, 907);
    }
}
