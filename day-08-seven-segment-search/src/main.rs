/*
--- Day 8: Seven Segment Search ---

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

fn parse_digits(input: &str) -> HashMap<usize, Vec<String>> {
    let mut dict = HashMap::new();
    for n in 2..=7 {
        dict.insert(n, vec![]);
    }

    let mut nums: Vec<String> = input
        .split_ascii_whitespace()
        .into_iter()
        .map(|s| sort_str_to_string(s))
        .collect();
    // nums.sort_by(|a, b| b.len().cmp(&a.len()));

    let lengths = nums.iter().map(|n| n.len());
    for (digit, len) in nums.iter().zip(lengths) {
        dict.get_mut(&len).unwrap().push(digit.to_owned());
    }

    dict
}

#[derive(Debug)]
struct Segment {
    digits: HashMap<usize, Vec<String>>,
    code: Vec<String>,
}
impl Segment {
    fn from_str(input: &str) -> Self {
        let parts: Vec<&str> = input
            .split(" | ")
            // .map(|s| s.to_owned())
            .collect();

        let digits = parse_digits(parts[0]);
        let mut code: Vec<_> = parts[1]
            .split_ascii_whitespace()
            .map(|s| s.to_owned())
            .collect();
        code.sort_by(|a, b| a.len().cmp(&b.len()));

        Self { digits, code }
    }
}

fn parse_input(input: &str) -> Vec<Segment> {
    input.lines().map(|line| Segment::from_str(line)).collect()
}

fn do_part_one(input: &str, digits_we_want: HashSet<usize>) -> usize {
    let segments = parse_input(input);
    // println!("{:?}", segments[0]);
    let v: Vec<_> = segments
        .into_iter()
        .map(|seg| seg.code)
        .flat_map(|codes| {
            codes
                .iter()
                .filter(|code| digits_we_want.contains(&code.len()))
                .cloned()
                .collect::<Vec<String>>()
        })
        .collect();
    // println!("{:?}", v);

    v.len()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

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
        let lengths_we_want: HashSet<usize> = HashSet::from_iter([2, 3, 4, 7,].iter().cloned());
        let result = do_part_one(EXAMPLE, lengths_we_want);
        assert_eq!(result, 26)
    }

    #[test]
    fn part1_full() {
        // these are...                                     one, seven, four, eight
        let lengths_we_want: HashSet<usize> = HashSet::from_iter([2, 3, 4, 7,].iter().cloned());
        let result = do_part_one(FULL, lengths_we_want);
        assert_eq!(result, 409)
    }
}
