#![allow(dead_code)]

mod command;
mod submarine_position;
mod operation;

use std::convert::{TryFrom, TryInto};

use command::Command;
use submarine_position::SubmarinePosition;

pub fn run_sub_directions(in_str: &str) -> SubmarinePosition {
    let vec_commands = in_str
        .lines()
        .filter_map(|line| Command::try_from(line).ok()) // look! Both TryFrom .. (see ln 27)
        .collect::<Vec<Command>>();
    let mut sub = SubmarinePosition::new();
    for cmd in vec_commands {
        sub.process_command(&cmd);
    }
    sub
}

pub fn run_sub_with_new_directions(in_str: &str) -> SubmarinePosition {
    let vec_commands = in_str
        .lines()
        .filter_map(|line| line.try_into().ok()) // and TryInto! (from ln 15)
        .collect::<Vec<Command>>();
    let mut sub = SubmarinePosition::new();
    for cmd in vec_commands {
        sub.process_new_command(&cmd);
    }
    sub
}

pub const INPUT: &str = include_str!("../input.txt");

fn main() {
    let sub = run_sub_directions(INPUT);
    println!("final position: {:?}", sub);
    println!("multiplied: {}", sub.depth * sub.horizontal);

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
