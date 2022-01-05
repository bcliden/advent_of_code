use std::collections::HashMap;

type CrabVec = Vec<usize>;
type PositionMap = HashMap<usize, usize>;

#[derive(Debug, Clone)]
struct CrabSolution {
    horizontal_position: usize,
    fuel_required: usize,
}

fn parse_input(input: &str) -> CrabVec {
    input
        .trim()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn triangular_number(n: usize) -> usize {
    // 5 = 1 + 2 + 3 + 4 + 5
    // n(n+1) / 2
    (n * (n + 1)) / 2
}

fn run_crab_simulation<F>(input: &str, calc_fuel: F) -> CrabSolution
where
    F: Fn(&CrabVec, usize) -> usize,
{
    let crabs: CrabVec = parse_input(input);
    let local_max = crabs.iter().max().unwrap();
    let mut positions: PositionMap = HashMap::new();

    for n in 0..local_max.clone() {
        let fuel_used = calc_fuel(&crabs, n);
        let count = positions.entry(n).or_default();
        *count = fuel_used;
    }

    let (horizontal_position, fuel_required) = positions
        .iter()
        .min_by(|(_, fuel_a), (_, fuel_b)| fuel_a.cmp(fuel_b))
        .map(|(pos, fuel)| (pos.clone(), fuel.clone()))
        .unwrap();

    CrabSolution {
        horizontal_position,
        fuel_required,
    }
}

/// doing each position x each crab seems like bad O(n²) runtime
fn calculate_part_one(input: &str) -> CrabSolution {
    let fuel_calc = |crabs: &CrabVec, goal: usize| {
        crabs
            .iter()
            .map(|position| {
                if position == &goal {
                    return 0;
                }
                let max = position.max(&goal);
                let min = position.min(&goal);
                max - min
            })
            .sum()
    };
    run_crab_simulation(input, fuel_calc)
}

fn calculate_part_two(input: &str) -> CrabSolution {
    let fuel_calc = |crabs: &CrabVec, goal: usize| {
        crabs
            .iter()
            .map(|position| {
                if position == &goal {
                    return 0;
                }
                let max = position.max(&goal);
                let min = position.min(&goal);
                let distance = max - min;
                triangular_number(distance)
            })
            .sum()
    };
    run_crab_simulation(input, fuel_calc)
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{:?}", calculate_part_one(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";
    const FULL_INPUT: &str = include_str!("../input.txt");

    #[test]
    fn does_the_part1_example() {
        let solution = calculate_part_one(TEST_INPUT);
        assert_eq!(solution.fuel_required, 37);
    }

    #[test]
    fn does_the_part1_full_problem() {
        let solution = calculate_part_one(FULL_INPUT);
        assert_eq!(solution.fuel_required, 364898);
    }

    #[test]
    fn does_the_part2_example() {
        let solution = calculate_part_two(TEST_INPUT);
        assert_eq!(solution.fuel_required, 168);
    }

    #[test]
    fn does_the_part2_full_problem() {
        let solution = calculate_part_two(FULL_INPUT);
        assert_eq!(solution.fuel_required, 104149091);
    }
}
