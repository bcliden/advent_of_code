use std::collections::HashSet;

type Coords = (i32, i32);
type Floor = Vec<Vec<usize>>;
type VisitorLog = HashSet<Coords>;
type LowestLog = Vec<(Coords, usize)>;

fn get_square_value<'a>((row, col): &Coords, floor: &'a Floor) -> Option<&'a usize> {
    floor
        .get(row.clone() as usize)
        .map(|_row| _row.get(col.clone() as usize))
        .flatten()
}

fn neighboring_squares((row, col): Coords) -> Vec<Coords> {
    vec![
        (row, col - 1), // top
        (row, col + 1), // bottom
        (row - 1, col), // left
        (row + 1, col), // right
    ]
}

fn get_values_for(coords: &Vec<Coords>, floor: &Floor) -> Vec<(Coords, usize)> {
    coords
        .iter()
        .cloned()
        // get value for next squares
        .map(|c| (c, get_square_value(&c, floor)))
        // filter out missing ones
        .filter(|(_, val)| val.is_some())
        .map(|(c, v)| (c, v.map(|v| v.clone()).unwrap_or(9)))
        .collect()
}

fn visit(
    floor: &Floor,
    visited: &mut VisitorLog,
    lowest_points: &mut LowestLog,
    own_coord: Coords,
) {
    if visited.contains(&own_coord) {
        return;
    }

    let (row, col) = own_coord;
    let own_value = get_square_value(&own_coord, floor)
        .map(|v| v.clone())
        .unwrap_or(9);

    visited.insert((row, col));

    let all_coords = neighboring_squares(own_coord.clone());
    let coords_pairs = get_values_for(&all_coords, floor);

    let is_least = &own_value < coords_pairs.iter().map(|(_, v)| v).min().unwrap();
    if is_least {
        lowest_points.push((own_coord, own_value));
    }
}

/// Recursively mark current square and neighboring squares.
/// Records visited squares and squares within current basin,
/// finally returning the size of the basin.
fn mark_squares(
    floor: &Floor,
    visited: &mut VisitorLog,
    current_basin: &mut VisitorLog,
    own_coord: Coords,
) {
    if visited.contains(&own_coord) {
        return;
    }
    visited.insert(own_coord.clone());

    match get_square_value(&own_coord, floor) {
        Some(n) if *n == 9 => return, // quit if 9
        Some(_) => current_basin.insert(own_coord.clone()),
        None => return, // quit if None
    };

    neighboring_squares(own_coord)
        .into_iter()
        .for_each(|c| mark_squares(floor, visited, current_basin, c));
}

fn get_basin_size(floor: &Floor, own_coord: Coords) -> usize {
    let mut visited: VisitorLog = HashSet::new(); // per-basin-search level
    let mut current_basin: VisitorLog = HashSet::new();

    mark_squares(floor, &mut visited, &mut current_basin, own_coord);

    current_basin.iter().count()
}

fn does_part2(input: &str) -> usize {
    let floor: Floor = parse_input(input);
    let mut visited: VisitorLog = HashSet::new();
    let mut basin_origins: LowestLog = vec![];

    for (row, _) in floor.iter().enumerate() {
        for (col, _) in floor[row].iter().enumerate() {
            visit(
                &floor,
                &mut visited,
                &mut &mut basin_origins,
                (row as i32, col as i32),
            );
        }
    }

    let mut basin_sizes: Vec<_> = basin_origins
        .into_iter()
        .map(|(c, _)| c) // drop the values, we don't need 'em
        .map(|c| get_basin_size(&floor, c))
        .collect();
    basin_sizes.sort(); // just reverse iterate below instead of sorting DESC
    basin_sizes.into_iter().rev().take(3).fold(1, |acc, n| acc * n)
}

fn does_part1(input: &str) -> usize {
    let floor: Floor = parse_input(input);
    let mut visited: VisitorLog = HashSet::new();
    let mut lowest_point_values: LowestLog = vec![];

    for (row, _line) in floor.iter().enumerate() {
        for (col, _value) in floor[row].iter().enumerate() {
            visit(
                &floor,
                &mut visited,
                &mut lowest_point_values,
                (row as i32, col as i32),
            );
        }
    }

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
        floor[n] = line
            .trim()
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
    println!("total basin factor calculation: {}", basin_sizes);
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
        assert_eq!(total_risk, 1134);
    }

    #[test]
    fn part2_full() {
        let total_risk = does_part2(FULL);
        /*
            Wrong answers discovered:
            -
        */
        assert_eq!(total_risk, 902880);
    }
}
