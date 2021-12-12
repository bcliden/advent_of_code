use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Eq)]
enum LineStatus {
    Complete,
    Incomplete,
    Illegal(char),
}

/// Compare enums by category, not by value
fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

fn parse_input(input: &str) -> Vec<VecDeque<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part_one(input: &str) -> usize {
    let scores: HashMap<char, usize> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let closing_pairs: HashMap<char, char> =
        HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);

    let mut results: Vec<LineStatus> = vec![];
    let data = parse_input(input);

    for mut line in data {
        let mut queue: VecDeque<char> = VecDeque::new();
        loop
        /* chars in line */
        {
            // println!("Reading line {:?}", line);
            // println!("With queue of {:?}", queue);
            let ch = line.pop_front();
            // println!("Just popped char {:?}", ch);

            if ch.is_none() {
                if queue.is_empty() {
                    results.push(LineStatus::Complete);
                } else {
                    results.push(LineStatus::Incomplete);
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
                    if queue.back() == Some(&closing_pairs[&ch]) {
                        queue.pop_back(); // dump the last item
                    } else {
                        results.push(LineStatus::Illegal(ch));
                        break; // exit reading line
                    }
                }
                ']' => {
                    if queue.back() == Some(&closing_pairs[&ch]) {
                        queue.pop_back(); // remove the paired char
                    } else {
                        results.push(LineStatus::Illegal(ch));
                        break; // exit reading line
                    }
                }
                '}' => {
                    if queue.back() == Some(&closing_pairs[&ch]) {
                        queue.pop_back(); // remove the paired char
                    } else {
                        results.push(LineStatus::Illegal(ch));
                        break; // exit reading line
                    }
                }
                '>' => {
                    if queue.back() == Some(&closing_pairs[&ch]) {
                        queue.pop_back(); // remove the paired char
                    } else {
                        results.push(LineStatus::Illegal(ch));
                        break; // exit reading line
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    results
        .into_iter()
        .filter(|r| variant_eq(r, &LineStatus::Illegal('_')))
        .map(|r| {
            if let LineStatus::Illegal(n) = r {
                return n; // get the value from the enum!
            } else {
                unreachable!();
            }
        })
        .map(|c| scores.get(&c).map(|n| n.clone()).unwrap_or(0)) // get score from dict
        .sum()
}

fn main() {
    const FULL: &str = include_str!("../input.txt");
    let score: usize = part_one(FULL);
    println!("Score for part one: {}", score);
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
}
