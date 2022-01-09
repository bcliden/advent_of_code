fn main() {
    println!("Hello, world!");
}

const FULL: &str = include_str!("input.txt");
const EXAMPLE: &str = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

mod part_one {
    use im::HashSet;
    use std::fmt;

    pub struct Answers(HashSet<u8>);

    impl fmt::Debug for Answers {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for &answer in &self.0 {
                write!(f, "{}", answer as char)?;
            }
            Ok(())
        }
    }

    fn calc(s: &str) -> usize {
        let answer: usize = s
            .split("\n\n")
            .map(|group| {
                HashSet::<u8>::unions(
                    group
                        .lines()
                        .map(|line| line.as_bytes().iter().copied().collect()),
                )
                .len()
            })
            .sum();

        answer
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_example() {
            assert_eq!(calc(super::super::EXAMPLE), 11);
        }

        #[test]
        fn test_full() {
            assert_eq!(calc(super::super::FULL), 6291);
        }
    }
}

mod part_two {
    use im::HashSet;
    use std::fmt;

    pub struct Answers(HashSet<u8>);

    impl fmt::Debug for Answers {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for &answer in &self.0 {
                write!(f, "{}", answer as char)?;
            }
            Ok(())
        }
    }

    fn calc(s: &str) -> usize {
        s.split("\n\n")
            .map(|group| {
                group
                    .lines()
                    .map(|line| line.as_bytes().iter().copied().collect())
                    .reduce(|acc: HashSet<u8>, x| acc.intersection(x))
                    .unwrap_or_default()
                    .len()
            })
            .sum()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_example() {
            assert_eq!(calc(super::super::EXAMPLE), 6);
        }

        #[test]
        fn test_full() {
            assert_eq!(calc(super::super::FULL), 3052);
        }
    }
}
