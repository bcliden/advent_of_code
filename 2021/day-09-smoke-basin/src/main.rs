use std::collections::HashSet;

// ==================[ Common ]====================

type Coords = (i32, i32);
type CaveFloor = Vec<Vec<usize>>;
type VisitorLog = HashSet<Coords>;
type LowestLog = Vec<(Coords, usize)>;

/// Parse input into a 2d array of usize values
fn parse_input(input: &str) -> CaveFloor {
    let width = input.lines().next().unwrap().trim().len();
    let height = input.lines().count();

    let mut floor: CaveFloor = Vec::with_capacity(height);
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

/// Get value for at a given coordinate
fn get_coord_value<'a>((row, col): &Coords, floor: &'a CaveFloor) -> Option<&'a usize> {
    floor
        .get(row.clone() as usize)
        .map(|_row| _row.get(col.clone() as usize))
        .flatten()
}

/// Generate neighboring (top, bot, left, right) coordinates from one central coord
fn neighboring_coords((row, col): Coords) -> Vec<Coords> {
    vec![
        (row, col - 1), // top
        (row, col + 1), // bottom
        (row - 1, col), // left
        (row + 1, col), // right
    ]
}

/// Walk over entire grid, finding the lowest point of basins in grid
fn find_basin_origins(floor: &CaveFloor) -> LowestLog {
    let mut visited: VisitorLog = HashSet::new();
    let mut basin_origins: LowestLog = vec![]; // to be filled by crawling fn

    for (row, _) in floor.iter().enumerate() {
        for (col, _) in floor[row].iter().enumerate() {
            visit(
                &floor,
                &mut visited,
                &mut basin_origins,
                (row as i32, col as i32),
            );
        }
    }
    basin_origins
}

// ==================[ Part 1 ]====================

/// Find basin origins, then sum the values (+1) to generate the risk level
fn does_part1(input: &str) -> usize {
    let floor: CaveFloor = parse_input(input);
    find_basin_origins(&floor)
        .into_iter()
        .map(|(_, v)| v + 1)
        .sum()
}

/// visit a given coordinate, marking as 'inserted', and checking if it's a valid lowest point
fn visit(
    floor: &CaveFloor,
    visited: &mut VisitorLog,
    lowest_points: &mut LowestLog,
    own_coord: Coords,
) {
    if visited.contains(&own_coord) {
        return;
    }

    let (row, col) = own_coord;
    let own_value = get_coord_value(&own_coord, floor)
        .map(|v| v.clone())
        .unwrap_or(9);

    visited.insert((row, col));

    let all_coords = neighboring_coords(own_coord.clone());
    let coords_pairs = get_batch_coord_values(&all_coords, floor);

    let minimum_value = coords_pairs.iter().map(|(_, v)| v).min().unwrap();
    if own_value < *minimum_value {
        lowest_points.push((own_coord, own_value));
    }
}


/// Get all valid values for a list of coordinates
/// in the form of (Coordinate, Value)
fn get_batch_coord_values(coords: &Vec<Coords>, floor: &CaveFloor) -> Vec<(Coords, usize)> {
    coords
        .iter()
        .cloned()
        // get value for next squares
        .map(|c| (c, get_coord_value(&c, floor)))
        // filter out missing ones
        .filter(|(_, val)| val.is_some())
        .map(|(c, v)| (c, v.map(|v| v.clone()).unwrap_or(9)))
        .collect()
}

// ==================[ Part 2 ]====================

/// Using basin origins (discovered in pt1),
/// find SIZE of basin and multiply the top 3 together.
fn does_part2(input: &str) -> usize {
    let floor: CaveFloor = parse_input(input);
    let mut basin_sizes: Vec<_> = find_basin_origins(&floor)
        .into_iter()
        .map(|(c, _)| c) // drop the values, we don't need 'em
        .map(|c| get_basin_size(&floor, c))
        .collect();
    basin_sizes.sort(); // just reverse iterate below instead of sorting DESC
    basin_sizes
        .into_iter()
        .rev()
        .take(3)
        .fold(1, |acc, n| acc * n)
}

/// From a base coordinate, recursively gather squares in a basin
/// until either a 9 or end of file.
///
/// Returns number of squares within basin
fn get_basin_size(floor: &CaveFloor, own_coord: Coords) -> usize {
    let mut visited: VisitorLog = HashSet::new(); // per-basin search level
    let mut current_basin: VisitorLog = HashSet::new(); // to be filled by crawling fn
    count_squares(floor, &mut visited, &mut current_basin, own_coord);
    current_basin.iter().count()
}

/// Recursively mark current square and neighboring squares.
/// Records visited squares and squares within current basin,
/// finally returning the size of the basin.
fn count_squares(
    floor: &CaveFloor,
    visited: &mut VisitorLog,
    current_basin: &mut VisitorLog, // really could just be a usize, but keeping the coords is nice
    own_coord: Coords,
) {
    if visited.contains(&own_coord) {
        return;
    }
    visited.insert(own_coord.clone());

    match get_coord_value(&own_coord, floor) {
        Some(n) if *n == 9 => return, // quit if 9
        Some(_) => current_basin.insert(own_coord.clone()),
        None => return, // quit if None
    };

    for coord in neighboring_coords(own_coord) {
        count_squares(floor, visited, current_basin, coord);
    }
}

// ===============[ The rest of it ]==================

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
