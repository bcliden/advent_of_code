/*
--- Day 8: Seven Segment Search ---

# Part 1

  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg


 Notes:
 - all wires are connected to segments *randomly*
 - all wires are within their correct segment

 For each display:
 1. note all 10 unique signal patterns
 2. write a single four digit output value

 translate pattern => output

 FOR EXAMPLE:

 signal pattern (Numbers 0-9 encoded)                          output value (Values to be shown)
 acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |  cdfeb fcadb cdfeb cdbaf

EASY INPUTS:

One:    2 segments
Four:   4 segments
Seven:  3 segments
Eight:  7 segments

# Part 2
The signal wires are in the following positions:

 dddd
e    a
e    a
 ffff
g    b
g    b
 cccc

*/
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

fn sort_str_to_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    String::from_iter(chars)
}

fn parse_signals(input: &str) -> HashMap<usize, Vec<HashSet<char>>> {
    let mut dict = HashMap::new();
    for n in 2..=7 {
        dict.insert(n, vec![]);
    }

    let mut nums: Vec<_> = input
        .split_ascii_whitespace()
        .into_iter()
        .map(|s| sort_str_to_string(s))
        .collect();
    // nums.sort_by(|a, b| b.len().cmp(&a.len()));

    let lengths = nums.iter().map(|n| n.len());
    for (digit, len) in nums.iter().zip(lengths) {
        dict.get_mut(&len)
            .unwrap()
            .push(HashSet::from_iter(digit.chars()));
    }

    dict
}

#[derive(Debug)]
struct Segment {
    signals: HashMap<usize, Vec<HashSet<char>>>,
    outputs: Vec<HashSet<char>>,
}
impl Segment {
    fn from_str(input: &str) -> Self {
        let parts: Vec<&str> = input.split(" | ").collect();

        let signals = parse_signals(parts[0]);
        let outputs: Vec<_> = parts[1]
            .split_ascii_whitespace()
            .map(|s| HashSet::from_iter(s.chars()))
            .collect();

        Self { signals, outputs }
    }
}

fn parse_input(input: &str) -> Vec<Segment> {
    input.lines().map(|line| Segment::from_str(line)).collect()
}

fn do_part_one(input: &str) -> usize {
    let lengths_we_want = [2, 3, 4, 7];
    parse_input(input)
        .into_iter()
        .map(|seg| seg.outputs)
        .flat_map(|codes| {
            codes
                .iter()
                .filter(|code| lengths_we_want.contains(&code.len()))
                .cloned()
                .map(|s| s.iter().cloned().collect::<String>())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<_>>()
        .len()
}

/// This very mess maps signals into numbers
fn map_signal_to_number(signals: HashMap<usize, Vec<HashSet<char>>>) -> HashMap<char, HashSet<char>> {
    let mut true_digits = HashMap::new();

        // unique length segmented numbers
        let one = signals.get(&2).unwrap().get(0).unwrap();
        true_digits.insert('1', one.clone());
        let four = signals.get(&4).unwrap().get(0).unwrap().clone();
        true_digits.insert('4', four.clone());
        let seven = signals.get(&3).unwrap().get(0).unwrap().clone();
        true_digits.insert('7', seven.clone());
        let eight = signals.get(&7).unwrap().get(0).unwrap().clone();
        true_digits.insert('8', eight.clone());

        // five length segments: 2, 3, 5
        let five_length_signals = signals.get(&5).unwrap();

        // of all the five digit signals, only 3 has bock UR and LR lit (same mapping as ONE)
        let three = five_length_signals
            .iter()
            .find(|set| one.is_subset(set))
            .unwrap();
        true_digits.insert('3', three.clone());

        // Five is the only digit to have up-left and mid segments
        let ul_and_mid_section: HashSet<char> = four.difference(one).cloned().collect();
        let five = five_length_signals
            .iter()
            .find(|set| ul_and_mid_section.is_subset(set))
            .unwrap();
        true_digits.insert('5', five.clone());

        // find two... it's not 3 or 5 :)
        let two = five_length_signals
            .iter()
            .find(|&set| set != three && set != five)
            .unwrap();
        true_digits.insert('2', two.clone());

        // six length segments: 6, 8, 9
        let six_length_signals = signals.get(&6).unwrap();

        // Six has the open UR segment... detect that
        let ur_section: HashSet<char> = three.difference(five).cloned().collect();
        let six = six_length_signals
            .iter()
            .find(|set| {
                // if eight minus the set == just the UR section
                let current_set_diff = eight.difference(set).cloned().collect::<HashSet<_>>();
                current_set_diff == ur_section
            })
            .unwrap();
        true_digits.insert('6', six.clone());

        // build Nine by doing a union of Five and One
        let nine: HashSet<_> = five.union(one).cloned().collect();
        true_digits.insert('9', nine.clone());

        let zero = six_length_signals.iter()
            .find(|&set| set != six && set != &nine).unwrap();
        true_digits.insert('0', zero.clone());

        true_digits
}

fn do_part_two(input: &str) -> usize {
    let segments = parse_input(input);
    let mut numbers_vec: Vec<usize> = vec![];

    for segment in segments.into_iter() {
        let digit_mapping = map_signal_to_number(segment.signals);
        // ok, we have every digit mapped. now what? Map code to numbers

        let tuples: Vec<(char, HashSet<char>)> = digit_mapping.into_iter().collect();

        let number_as_string: String = segment.outputs.iter()
            .map(|set| tuples.iter()
                .find(|(_, mapped_set)| set == mapped_set).unwrap()
                // map a set to a tuple of (number, signal)
            ).map(|(c, _)| c)
            .cloned()
            .collect();

        numbers_vec.push(number_as_string.parse().unwrap());
    }

    numbers_vec.iter().sum()
}

fn main() {
    const FULL: &str = include_str!("../input.txt");

    let result = do_part_one(FULL);
    assert_eq!(result, 409);

    let result = do_part_two(FULL);
    assert_eq!(result, 1024649);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    const FULL: &str = include_str!("../input.txt");

    #[test]
    fn part1_example() {
        let result = do_part_one(EXAMPLE);
        assert_eq!(result, 26)
    }

    #[test]
    fn part1_full() {
        let result = do_part_one(FULL);
        assert_eq!(result, 409)
    }

    #[test]
    fn part2_example() {
        let result = do_part_two(EXAMPLE);
        assert_eq!(result, 61229)
    }
    #[test]
    fn part2_full() {
        let result = do_part_two(FULL);
        assert_eq!(result, 1024649)
    }
}
