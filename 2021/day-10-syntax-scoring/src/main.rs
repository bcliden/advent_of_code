use phf::phf_map;
use std::collections::{HashMap, VecDeque};

enum Status {
    Complete,
    Incomplete,
    Illegal(char),
}

struct Line {
    chars_left: VecDeque<char>,
    status: Status,
}

// compile-time const map
const CLOSE_BRACE_TO_OPEN: phf::Map<char, char> = phf_map! {
    ')' => '(',
    ']'=> '[' ,
    '}'=> '{',
    '>'=> '<',
};

// compile-time const map
const OPEN_BRACE_TO_CLOSED: phf::Map<char, char> = phf_map! {
     '('  => ')',
     '[' => ']' ,
     '{' => '}',
     '<' => '>',
};

/// Compare enums by category, not by value
fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

fn parse_input(input: &str) -> Vec<VecDeque<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_solutions(data: Vec<VecDeque<char>>) -> Vec<Line> {
    let mut results: Vec<Line> = vec![];
    for mut line in data {
        let mut queue: VecDeque<char> = VecDeque::new();
        loop
        /* through chars in line */
        {
            let ch = line.pop_front();
            if ch.is_none() {
                if queue.is_empty() {
                    results.push(Line {
                        chars_left: queue,
                        status: Status::Complete,
                    });
                } else {
                    results.push(Line {
                        chars_left: queue,
                        status: Status::Incomplete,
                    });
                }
                break; // exit reading line
            }

            let ch = ch.unwrap();
            match ch {
                '(' => queue.push_back(ch),
                '[' => queue.push_back(ch),
                '{' => queue.push_back(ch),
                '<' => queue.push_back(ch),
                ')' => {
                    if queue.back() == Some(&CLOSE_BRACE_TO_OPEN[&ch]) {
                        queue.pop_back(); // remove the paired char
                    } else {
                        results.push(Line {
                            chars_left: queue,
                            status: Status::Illegal(ch),
                        });
                        break; // exit reading line
                    }
                }
                ']' => {
                    if queue.back() == Some(&CLOSE_BRACE_TO_OPEN[&ch]) {
                        queue.pop_back(); // remove the paired char
                    } else {
                        results.push(Line {
                            chars_left: queue,
                            status: Status::Illegal(ch),
                        });
                        break; // exit reading line
                    }
                }
                '}' => {
                    if queue.back() == Some(&CLOSE_BRACE_TO_OPEN[&ch]) {
                        queue.pop_back(); // remove the paired char
                    } else {
                        results.push(Line {
                            chars_left: queue,
                            status: Status::Illegal(ch),
                        });
                        break; // exit reading line
                    }
                }
                '>' => {
                    if queue.back() == Some(&CLOSE_BRACE_TO_OPEN[&ch]) {
                        queue.pop_back(); // remove the paired char
                    } else {
                        results.push(Line {
                            chars_left: queue,
                            status: Status::Illegal(ch),
                        });
                        break; // exit reading line
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    results
}

fn part_one(input: &str) -> usize {
    let point_values: HashMap<char, usize> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let data = parse_input(input);
    let mut solutions = get_solutions(data);

    // retain ONLY illegal lines
    solutions.retain(|line| variant_eq(&line.status, &Status::Illegal('_')));

    solutions
        .into_iter()
        .map(|line| {
            if let Status::Illegal(n) = line.status {
                n // get the value from the enum!
            } else {
                unreachable!();
            }
        })
        .map(|c| {
            point_values
                .get(&c) // get point value from dictionary
                .copied()
                .unwrap_or(0)
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let point_values: HashMap<char, usize> =
        HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let data = parse_input(input);
    let mut results = get_solutions(data);

    // retain ONLY incomplete lines
    results.retain(|line| variant_eq(&line.status, &Status::Incomplete));

    // convert remaining queues into solutions
    let solutions = results.iter().map(|line| {
        line.chars_left
            .iter()
            .map(|c| OPEN_BRACE_TO_CLOSED[c])
            .rev()
            .collect()
    });

    // convert braces to scores
    let scores =
        solutions.map(|chars: Vec<char>| chars.into_iter().map(|c| point_values[&c]).collect());

    // calculate the score per line
    let mut calculated_scores: Vec<usize> = scores
        .map(|scores_per_vec: Vec<usize>| {
            scores_per_vec
                .into_iter()
                .fold(0, |acc_score, next_score| (acc_score * 5) + next_score)
        })
        .collect();
    calculated_scores.sort_unstable();
    calculated_scores[calculated_scores.len() / 2] // the median score. thanks integer arithmetic!
}

fn main() {
    const FULL: &str = include_str!("../input.txt");
    let score: usize = part_one(FULL);
    println!("Score for part one: {}", score);
    let score: usize = part_two(FULL);
    println!("Score for part two: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;
    const FULL: &str = include_str!("../input.txt");

    #[test]
    fn does_part_one_example() {
        let result = part_one(EXAMPLE);
        assert_eq!(result, 26397);
    }

    #[test]
    fn does_part_one_full() {
        let result = part_one(FULL);
        assert_eq!(result, 315693);
    }

    #[test]
    fn does_part_two_example() {
        let result = part_two(EXAMPLE);
        assert_eq!(result, 288957);
    }

    #[test]
    fn does_part_two_full() {
        let result = part_two(FULL);
        assert_eq!(result, 1870887234);
    }
}
