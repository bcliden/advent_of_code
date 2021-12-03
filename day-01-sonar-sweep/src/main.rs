#![allow(dead_code)]
use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = include_str!("../input.txt");
    println!("first solution: {}", calculate_first_solution(s));
    println!("second solution: {}", calculate_second_solution(s));
    Ok(())
}

/// First go at it... not very rusty
fn calculate_first_solution_old (s: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let mut count = 0;

    let lines: Vec<u32> = s.lines()
        .filter_map(|n| n.parse::<u32>().ok())
        .collect();

    for (index, current) in lines.iter().enumerate() {
        if let Some(prev) = lines.get(index.saturating_sub(1)) {
            if current > prev {
                count += 1;
            }
        }
    }

    Ok(count)
}

/// Second attempt after reading about IterTools. Very helpful!
fn calculate_first_solution (s: &str) -> usize {
     s.lines()
        .filter_map(|n| n.parse::<usize>().ok())
        .tuple_windows::<(_, _)>()
        .filter(|(prev, current)| current > prev )
        .count()
}

fn calculate_second_solution (s: &str) -> usize {
    s.lines()
    .filter_map(|n| n.parse::<usize>().ok())
    // group into triplets and sum them to get each window val
    .tuple_windows::<(_, _, _)>()
    .map(|triplet| triplet.0 + triplet.1 + triplet.2)
    // then group into pairs to compare window sums
    .tuple_windows::<(_, _)>()
    .filter(|(prev, current)| current > prev )
    .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_the_first_solution() {
        let s = include_str!("../input.txt");
        let count = calculate_first_solution(s);
        assert_eq!(count, 1722);
    }

    #[test]
    fn it_finds_the_second_solution() {
        let s = include_str!("../input.txt");
        let count = calculate_second_solution(s);
        assert_eq!(count, 1748);
    }
}
