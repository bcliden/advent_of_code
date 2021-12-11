use std::collections::HashSet;

type Coords = (i32, i32);
type Floor = Vec<Vec<usize>>;
type VisitorLog = HashSet<Coords>;
type LowestLog = Vec<(Coords, usize)>;

fn get_square_value<'a> ((row, col): &Coords, floor: &'a Floor) -> Option<&'a usize>  {
    floor
        .get(row.clone() as usize)
        .map(|_row| _row.get(col.clone() as usize))
        .flatten()
}

fn visit(floor: &Floor, visited: &mut VisitorLog, lowest_points: &mut LowestLog, own_coord: Coords) {
    if visited.contains(&own_coord) {
        return;
    }



    let (row, col) = own_coord;
    let own_value= get_square_value(&own_coord, floor).map(|v| v.clone()).unwrap_or(9);

    // println!("[i] Visiting {:?} -> {}", (row, col), own_value);
    visited.insert((row, col));

    let all_coords = vec![
        (row, col - 1), // top
        (row, col + 1), // bottom
        (row - 1, col), // left
        (row + 1, col), // right
    ];

    let coords_pairs: Vec<(Coords, usize)> = all_coords
        .into_iter()
        // get value for next squares
        .map(|c| (c, get_square_value(&c, floor)))
        // filter out missing ones
        .filter(|(_, val)| val.is_some())
        .map(|(c, v)| (c, v.map(|v| v.clone()).unwrap_or(9)))
        .collect();

    let is_least = &own_value < coords_pairs.iter().map(|(_, v)| v).min().unwrap();

    // let lower_pairs: Vec<_> = coords_pairs
    //     .iter().cloned()
    //     .filter(|(_other_coord, other_val)| {
    //         // println!(
    //         //     "\t[i] comparing current {:?}[{}] to next {:?}[{}] and returning lower={}",
    //         //     own_coord,
    //         //     &own_value,
    //         //     other_coord,
    //         //     other_val,
    //         //     other_val < &own_value
    //         // );
    //         /*   9
    //             999
    //              9    should the center be lower? no. current val (9) should be LESS THAN other val
    //         */    
    //         // in which our current cell is strictly LESS THAN the other cell
    //         &own_value > other_val
    //     })
    //     .collect();
    // let higher_pairs: Vec<_> = coords_pairs
    //     .iter().cloned()
    //     .filter(|(_other_coord, other_val)| {
    //         // println!(
    //         //     "\t[i] comparing current {:?}[{}] to next {:?}[{}] and returning lower={}",
    //         //     own_coord,
    //         //     &own_value,
    //         //     other_coord,
    //         //     other_val,
    //         //     other_val < &own_value
    //         // );
    //         // in which our current cell is strictly LESS THAN the other cell
    //         &own_value < other_val
    //     })
    //     .collect();
    // println!("[i] For {:?}[{}], found ", own_coord, own_value);
    // println!("\t[i] Higher pairs: {:?}", higher_pairs);
    // println!("\t[i] Lower pairs: {:?}", lower_pairs);
    // if lower_pairs.is_empty() {
    if is_least {
        // lowest_points.insert(own_coord);
        // or just add the value? would need a signature change
        // println!("[!] lowest area detected: {:?} -> {}", own_coord, own_value);
        lowest_points.push((own_coord, own_value));
    }

    // Turns out, I didn't need to recurse like this... who'd a thunk it?

    // } else {
    //     // visit any NON visited lower pairs
    //     let non_visited_pairs: Vec<_> = coords_pairs
    //         .into_iter()
    //         .filter(|(c, _)| !visited.contains(c))
    //         .collect();
    //     for (coord, _value) in non_visited_pairs {
    //         visit(floor, visited, lowest_points, coord);
    //     }
    // }
}

fn does_part2(input: &str) -> usize {
    let floor: Floor = parse_input(input);
    let mut visited: VisitorLog = HashSet::new();
    let mut lowest_point_values: LowestLog = vec![];

    // just recursively do it
    for (row, _line) in floor.iter().enumerate() {
        for (col, _value) in floor[row].iter().enumerate() {
            visit(&floor, &mut visited, &mut lowest_point_values, (row as i32, col as i32));
        }
    }

    println!("Basins located at: {:?}", lowest_point_values);

    0
}

fn does_part1(input: &str) -> usize {
    let floor: Floor = parse_input(input);
    let mut visited: VisitorLog = HashSet::new();
    let mut lowest_point_values: LowestLog = vec![];

    // just recursively do it
    for (row, _line) in floor.iter().enumerate() {
        for (col, _value) in floor[row].iter().enumerate() {
            visit(&floor, &mut visited, &mut lowest_point_values, (row as i32, col as i32));
        }
    }

    println!("All lowest points: {:?}", lowest_point_values);

    lowest_point_values.into_iter().map(|(_, v)| v + 1).sum()
}

fn parse_input(input: &str) -> Floor {
    let width = input.lines().next().unwrap().trim().len();
    let height = input.lines().count();

    let mut floor: Floor = Vec::with_capacity(height);
    for _ in 0..height {
        floor.push(Vec::with_capacity(width))
    }

    for (n, line) in input.lines().enumerate() {
        floor[n] = line.trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
    }

    floor
}

fn main() {
    const FULL: &str = include_str!("../input.txt");
    
    let total_risk = does_part1(FULL);
    println!("Total floor risk: {}", total_risk);

    let basin_sizes = does_part2(FULL);
    assert_eq!(basin_sizes, 496);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;
    const FULL: &str = include_str!("../input.txt");

    #[test]
    fn part1_example() {
        let total_risk = does_part1(EXAMPLE);
        assert_eq!(total_risk, 15);
    }

    #[test]
    fn part1_full() {
        let total_risk = does_part1(FULL);
        /*
            Wrong answers discovered: 
            - 1716
        */
        assert_eq!(total_risk, 496);
    }

    #[test]
    fn part2_example() {
        let total_risk = does_part2(EXAMPLE);
        assert_eq!(total_risk, 15);
    }

    #[test]
    fn part2_full() {
        let total_risk = does_part2(FULL);
        /*
            Wrong answers discovered: 
            - 
        */
        assert_eq!(total_risk, 496);
    }
}
