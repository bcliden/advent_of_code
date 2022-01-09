use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum InstructionKind {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    kind: InstructionKind,
    operand: isize,
}

type Program = Vec<Instruction>;

fn parse_program(input: &str) -> Program {
    input
        .lines()
        .map(|l| {
            let mut tokens = l.split(' ');
            Instruction {
                kind: match tokens.next() {
                    Some(tok) => match tok {
                        "nop" => InstructionKind::Nop,
                        "acc" => InstructionKind::Acc,
                        "jmp" => InstructionKind::Jmp,
                        _ => panic!("unknown instruction kind {}", tok),
                    },
                    None => panic!("For line {}, expected instruction kind", l),
                },
                operand: match tokens.next() {
                    Some(tok) => tok.parse().unwrap(),
                    None => panic!("For line {}, expected operand", l),
                },
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, Default)]
struct State {
    /// Program Counter
    pc: usize,
    /// Accumulator
    acc: isize,
}

impl State {
    fn next(self, program: &Program) -> Option<Self> {
        if !(0..program.len()).contains(&self.pc) {
            // if pc is out of bounds
            return None;
        }

        let ins = program[self.pc];
        Some(match ins.kind {
            InstructionKind::Nop => Self {
                pc: self.pc + 1,
                ..self
            },
            InstructionKind::Acc => Self {
                pc: self.pc + 1,
                acc: self.acc + ins.operand,
            },
            InstructionKind::Jmp => Self {
                pc: (self.pc as isize + ins.operand).try_into().unwrap(),
                ..self
            },
        })
    }
}

fn solve_part_one(input: &str) -> isize {
    let program = parse_program(input);

    // iterator that calls the closure every time
    let mut iter = itertools::iterate(State::default(), |s| s.next(&program).unwrap());
    let mut set: HashSet<usize> = Default::default();

    // then, iterate until the hashset insert returns false
    let answer = iter.find(|state| !set.insert(state.pc)).unwrap();

    // there's your answer!
    answer.acc
}

// Part Two additions
// need to flip one instruction of nop <-> jmp to terminate program correctly

fn flip_kind(kind: &mut InstructionKind) {
    *kind = match *kind {
        InstructionKind::Jmp => InstructionKind::Nop,
        InstructionKind::Nop => InstructionKind::Jmp,
        x => x,
    }
}

/// Use this to log out the variant that finishes the fastest... then use that to solve the below parts
fn find_variant(program: &Program) {
    let mut variants: Vec<_> = program
        .iter()
        .enumerate()
        .filter_map(|(index, ins)| match ins.kind {
            InstructionKind::Jmp | InstructionKind::Nop => Some(index),
            _ => None,
        })
        .map(|i| {
            let mut variant = program.clone();
            flip_kind(&mut variant[i].kind);
            (i, variant)
        })
        .map(|(index, variant)| {
            itertools::iterate(Some(State::default()), move |state| {
                state
                    .unwrap_or_else(|| panic!("variant {} terminated!", index))
                    .next(&variant)
            })
        })
        .collect();

    loop {
        for v in &mut variants {
            v.next();
        }
    }
}

fn eval(program: &Program) -> Option<isize> {
    itertools::iterate(Some(State::default()), |state| {
        state.and_then(|state| state.next(program))
    })
    .while_some()
    .last()
    .map(|s| s.acc)
}

fn solve_part_two(input: &str) -> isize {
    let mut program = parse_program(input);
    flip_kind(&mut program[281].kind); // <-- determined from find_variant
    // dbg!(eval(&program));
    eval(&program).unwrap()
}

fn main() {
    // let program = parse_program(FULL);
    // find_variant(&program); // found idx 7 to be the fastest variant
    let acc = solve_part_two(FULL);
    println!("acc was {} at after flipping #281", acc)
}

const EXAMPLE: &str = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;
const FULL: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_part_one_example() {
        assert_eq!(solve_part_one(super::EXAMPLE), 5)
    }

    #[test]
    fn solve_part_one_full() {
        assert_eq!(solve_part_one(super::FULL), 1584)
    }

    #[test]
    fn solve_part_two_full() {
        assert_eq!(solve_part_two(super::FULL), 920)
    }
}
