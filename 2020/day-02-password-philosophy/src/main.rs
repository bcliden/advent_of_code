#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::bool_assert_comparison)]

mod part_one {
    use std::ops::RangeInclusive;

    #[derive(PartialEq, Debug)]
    pub struct PasswordPolicy {
        byte: u8,
        range: RangeInclusive<usize>,
    }

    impl PasswordPolicy {
        fn is_valid(&self, password: &str) -> bool {
            self.range.contains(
                &password
                    .as_bytes()
                    .iter()
                    .copied()
                    .filter(|&b| b == self.byte)
                    .count(),
            )
        }
    }

    fn parse_line(s: &str) -> anyhow::Result<(PasswordPolicy, &str)> {
        peg::parser! {
            grammar parser() for str {
                rule number() -> usize
                 = n:$(['0'..='9']+) { n.parse().unwrap() }

                rule range() -> RangeInclusive<usize>
                 = min:number() "-" max:number() { min..=max }

                rule byte() -> u8
                 = letter:$(['a'..='z']) { letter.as_bytes()[0] }

                rule password() -> &'input str
                 = letters:$([_]*) { letters }

                pub(crate) rule line() -> (PasswordPolicy, &'input str)
                 = range:range() " " byte:byte() ": " password:password() {
                    (PasswordPolicy { range, byte }, password)
                 }
            }
        }

        Ok(parser::line(s)?)
    }

    pub fn run(s: &str) -> usize {
        s.lines()
            .map(parse_line)
            .map(Result::unwrap)
            .filter(|(policy, password)| policy.is_valid(password))
            .count()
    }

    #[cfg(test)]
    mod part_one_tests {
        use super::*;

        #[test]
        fn test_is_valid() {
            let pp = PasswordPolicy {
                range: 1..=3,
                byte: b'a',
            };
            assert_eq!(pp.is_valid("zeus"), false, "no 'a's");
            assert_eq!(pp.is_valid("hades"), true, "single 'a'");
            assert_eq!(pp.is_valid("banana"), true, "three 'a's");
            assert_eq!(pp.is_valid("aaaah"), false, "too many 'a's");
        }

        #[test]
        fn test_parse() {
            assert_eq!(
                parse_line("1-3 a: banana").unwrap(),
                (
                    PasswordPolicy {
                        range: 1..=3,
                        byte: b'a'
                    },
                    "banana"
                )
            );
        }
    }
}

mod part_two {
    #[derive(PartialEq, Debug)]
    pub struct PasswordPolicy {
        byte: u8,
        positions: [usize; 2],
    }

    impl PasswordPolicy {
        fn is_valid(&self, password: &str) -> bool {
            self.positions
                .iter()
                .copied()
                .filter(|&index| password.as_bytes()[index] == self.byte)
                .count()
                == 1
        }
    }

    fn parse_line(s: &str) -> anyhow::Result<(PasswordPolicy, &str)> {
        /*
            Adjusted to find positions rather than range,
                as well as account for 1-indexed char positions
        */
        peg::parser! {
            grammar parser() for str {
                rule number() -> usize
                 = n:$(['0'..='9']+) { n.parse().unwrap() }

                rule position() -> usize
                 = n:number() { n - 1 }

                rule positions() -> [usize; 2]
                 = first:position() "-" second:position() { [first, second] }

                rule byte() -> u8
                 = letter:$(['a'..='z']) { letter.as_bytes()[0] }

                rule password() -> &'input str
                 = letters:$([_]*) { letters }

                pub(crate) rule line() -> (PasswordPolicy, &'input str)
                 = positions:positions() " " byte:byte() ": " password:password() {
                    (PasswordPolicy { positions, byte }, password)
                 }
            }
        }

        Ok(parser::line(s)?)
    }

    pub fn run(s: &str) -> usize {
        s.lines()
            .map(parse_line)
            .map(Result::unwrap)
            .filter(|(policy, password)| policy.is_valid(password))
            .count()
    }

    #[cfg(test)]
    mod part_two_tests {
        use super::*;

        #[test]
        fn test_is_valid() {
            let pp = PasswordPolicy {
                positions: [0, 2],
                byte: b'a',
            };
            assert_eq!(pp.is_valid("abcde"), true, "'a' in position 1");
            assert_eq!(pp.is_valid("bcade"), true, "'a' in position 3");
            assert_eq!(pp.is_valid("food"), false, "no 'a' whatsoever");
            assert_eq!(pp.is_valid("abacus"), false, "'a' in both positions");
        }

        #[test]
        fn test_parse() {
            assert_eq!(
                parse_line("1-3 a: banana").unwrap(),
                (
                    PasswordPolicy {
                        positions: [0, 2],
                        byte: b'a'
                    },
                    "banana"
                )
            );
        }
    }
}

// fn part_two(s: &str) -> usize {
//     s.lines()
//         .map(parse_line)
//         .map(Result::unwrap)
//         .filter(|(policy, password)| policy.is_valid(password))
//         .count()
// }

fn main() -> anyhow::Result<()> {
    // part one
    let count = part_one::run(include_str!("input.txt"));
    println!("{} passwords are valid", count);

    // part two
    let count = part_one::run(include_str!("input.txt"));
    println!("{} passwords are valid", count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#;

    #[test]
    fn part_1_example() {
        let count = part_one::run(EXAMPLE);
        assert_eq!(count, 2)
    }

    #[test]
    fn part_1_full() {
        let count = part_one::run(include_str!("input.txt"));
        assert_eq!(count, 519)
    }

    #[test]
    fn part_2_example() {
        let count = part_two::run(EXAMPLE);
        assert_eq!(count, 1)
    }

    #[test]
    fn part_2_full() {
        let count = part_two::run(include_str!("input.txt"));
        assert_eq!(count, 708)
    }
}

/*
Naive part_one parse_line function. Works great!

#[derive(thiserror::Error, Debug)]
enum ParseError {
    #[error("expected {0}")]
    Expected(&'static str),
}

fn parse_line(s: &str) -> anyhow::Result<(PasswordPolicy, &str)> {
    let (policy, password) = {
        let mut tokens = s.split(':');
        (
            tokens
                .next()
                .ok_or(ParseError::Expected("password policy"))?,
            tokens
                .next()
                .ok_or(ParseError::Expected("password"))?
                .trim(),
        )
    };

    let (range, byte) = {
        let mut tokens = policy.split(' ');
        (
            tokens.next().ok_or(ParseError::Expected("policy range"))?,
            tokens.next().ok_or(ParseError::Expected("policy byte"))?,
        )
    };

    let byte = if byte.as_bytes().len() == 1 {
        byte.as_bytes()[0]
    } else {
        return Err(ParseError::Expected("password policy byte to be exactly one byte").into());
    };

    let (min, max) = {
        let mut tokens = range.split('-');
        (
            tokens
                .next()
                .ok_or(ParseError::Expected("policy range (lower bound)"))?,
            tokens
                .next()
                .ok_or(ParseError::Expected("policy range (upper bound)"))?,
        )
    };

    let range = (min.parse()?)..=(max.parse()?);

    Ok((PasswordPolicy { range, byte }, password))
}



*/
