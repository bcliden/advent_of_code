#![allow(dead_code, unused_imports)]
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    // pt1
    let input = include_str!("input.txt");
    let pair = find_correct_pair(input)?;
    dbg!("part1", pair);

    // pt2
    let pair = find_correct_pair(input)?;
    dbg!("part2", pair);
    Ok(())
}

fn parse_input(s: &str) -> Result<Vec<i64>, std::num::ParseIntError> {
    s.split_ascii_whitespace()
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>() // using Result<Vec> here will return Err if ANY fail to parse, else Ok(Vec)
}

fn find_correct_pair(s: &str) -> anyhow::Result<(i64, i64)> {
    Ok(parse_input(s)?
            .into_iter()
            .tuple_combinations()
            .find(|(a, b)| a + b == 2020)
            .expect("no pair had a sum of 2020"))
}

fn find_correct_triplet(s: &str) -> anyhow::Result<(i64, i64, i64)> {
    Ok(parse_input(s)?
    .into_iter()
    .tuple_combinations()
    .find(|(a, b, c)| a + b + c == 2020)
    .expect("no triplet had a sum of 2020")
)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"1721
979
366
299
675
1456"#;
const FULL: &str = include_str!("input.txt");

    #[test]
    fn does_pt1_example() -> anyhow::Result<()> {
        let (a, b) = find_correct_pair(EXAMPLE)?;
        assert_eq!(a * b, 514579);
        Ok(())
    }

    #[test]
    fn does_pt1_full() -> anyhow::Result<()> {
        let (a, b) = find_correct_pair(FULL)?;
        assert_eq!(a * b, 793524);
        Ok(())
    }

    #[test]
    fn does_pt2_example() -> anyhow::Result<()> {
        let (a, b, c) = find_correct_triplet(EXAMPLE)?;
        assert_eq!(a * b * c, 241861950);
        Ok(())
    }

    #[test]
    fn does_pt2_full() -> anyhow::Result<()> {
        let (a, b, c) = find_correct_triplet(FULL)?;
        assert_eq!(a * b * c, 61515678);
        Ok(())
    }
}

/* unneeded historical cruft

// Itertools makes this obsolete
fn all_pairs(s: &[i64]) -> impl Iterator<Item = (i64, i64)> + '_ {
    // First pass:
    // let mut pairs: Vec<_> = Default::default();
    // for i in 0..s.len() {
    //     for j in 0..s.len() {
    //         pairs.push((s[i], s[j]));
    //     }
    // }
    // pairs

    // Second pass: (yeesh)
    s.iter()
        .copied()
        .enumerate()
        .map(move |(a_index, a)| {
            // for each A..
            s.iter()
                .copied()
                .enumerate()
                .filter_map(move |(b_index, b)| {
                    // ..and for each B, build Option

                    if a_index == b_index {
                        None
                    } else {
                        Some((a, b))
                    }
                })
        })
        .flatten()
}

// Itertools makes this obsolete
fn find_pair_whose_sum_is_2020(s: Vec<i64>) -> Option<(i64, i64)> {
    all_pairs(&s[..]).into_iter().find(|(a, b)| a + b == 2020)
}

*/