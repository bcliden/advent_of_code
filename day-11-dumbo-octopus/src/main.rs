use std::{collections::HashSet, fmt::Display};

// #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
// struct Octopus(usize);
// impl Octopus {
//     fn check_something() {}
// }
type Octopus = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coords(i32, i32);
impl Coords {
    fn to_tuple(&self) -> (i32, i32) {
        (self.0, self.1)
    }

    fn neighboring_coords(&self) -> Vec<Coords> {
        let (row, col) = self.to_tuple();
        vec![
            Coords(row - 1, col - 1), // top-left
            Coords(row - 1, col),     // top
            Coords(row - 1, col + 1), // top-right
            Coords(row, col - 1),     // left
            // center... our current point
            Coords(row, col + 1),     // right
            Coords(row + 1, col - 1), // bottom-left
            Coords(row + 1, col),     // bottom
            Coords(row + 1, col + 1), // bottom-right
        ]
    }
}
// this should cover both From and Into, right?
// impl From<(i32, i32)> for Coords {
//     fn from(other: (i32, i32)) -> Self {
//         Coords(other.0, other.1)
//     }
// }

#[derive(Debug)]
struct OceanFloor {
    grid: Vec<Vec<Octopus>>,
    width: usize,
    height: usize,
}
impl OceanFloor {
    fn step(&mut self) -> usize /* number of flashes */ {
        self.increase_all();
        self.flash_step()
    }

    // Step 1
    fn increase_all(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let coord = Coords(row as i32, col as i32);
                let cell = self.get_mut_cell(&coord);
                if let Some(cell) = cell {
                    *cell += 1;
                }
            }
        }
    }

    // fn increase_some(&mut self, coords: &Vec<Coords>) {
    //     for coord in coords {
    //         let cell = self.get_mut_cell(coord);
    //         if let Some(cell) = cell {
    //             *cell += 1;
    //         }
    //     }
    // }

    fn flash_many(&mut self, flashers: &[Coords], already_flashed: &mut HashSet<Coords>) -> Vec<Coords> {
        let mut new_flashers: Vec<Coords> = vec![];

        // for each flasher
        for coord in flashers {
            // if already visited, stop
            if already_flashed.contains(coord) {
                continue;
            }
            // else mark as visited
            already_flashed.insert(coord.clone());

            // get neighbors
            for neighbor in coord.neighboring_coords() {
                // and increase their count
                if let Some(cell) = self.get_mut_cell(&neighbor) {
                    *cell += 1;

                    // if this cell is higher than 9 (and hasn't previously flashed), now IT needs to flash
                    if *cell > 9 && !already_flashed.contains(&neighbor) {
                        // println!("Coord {:?} has val {}... flashing", coord, cell);
                        new_flashers.push(neighbor);
                    }
                }
            }
        }
        new_flashers
    }

    fn get_flashers(&self) -> Vec<Coords> {
        let mut flash_queue: Vec<Coords> = vec![];
        for row in 0..self.height {
            for col in 0..self.width {
                let coord = Coords(row as i32, col as i32);
                if let Some(cell) = self.read_cell(&coord) {
                    if *cell > 9 {
                        // println!("Coord {:?} has val {}... flashing", coord, cell);
                        flash_queue.push(coord);
                    }
                }
            }
        }
        flash_queue
    }

    // Step 2
    fn flash_step(&mut self) -> usize {
        let mut already_flashed: HashSet<Coords> = HashSet::new();

        let mut next_flashers = self.get_flashers();
        // println!("Starting with flashers: {:?}", next_flashers);
        while !next_flashers.is_empty() {
            // println!("{}", self);
            // flash current list, getting any new flashers
            let mut new_flashers = self.flash_many(&next_flashers, &mut already_flashed);
            // add all the current flashers into the 'visited' set
            // for c in next_flashers  {
            //     already_flashed.insert(c);
            // }
            // set next_flashers to be new_flashers, stripping any already flashed ones
            new_flashers.retain(|c| !already_flashed.contains(c));
            next_flashers = new_flashers;
        }

        // println!("Flashed all of {:?}", already_flashed);
        let flashed = self.get_flashers();
        let count = flashed.len();

        // finally, reset any flashed squares to 0
        // for c in already_flashed.iter() {
        for c in flashed.iter() {
            if let Some(cell) = self.get_mut_cell(c) {
                *cell = 0;
            }
        }

        // already_flashed.into_iter().count()
        count
    }

    fn read_cell(&self, c: &Coords) -> Option<&Octopus> {
        let (row, col) = c.to_tuple();
        self.grid
            .get(row as usize)
            .map(|_row| _row.get(col as usize))
            .flatten()
    }

    fn get_mut_cell(&mut self, c: &Coords) -> Option<&mut Octopus> {
        let (row, col) = c.to_tuple();
        self.grid
            .get_mut(row as usize)
            .map(|_row| _row.get_mut(col as usize))
            .flatten()
    }
}
impl Display for OceanFloor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.grid.iter() {
            writeln!(f, "{:?}", line)?;
        }
        Ok(())
    }
}
fn parse_input(input: &str) -> OceanFloor {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    let mut v: Vec<Vec<Octopus>> = vec![vec![Octopus::default(); width]; height];
    for (idx, line) in input.lines().enumerate() {
        v[idx] = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|n| n as usize)
            .collect()
    }
    OceanFloor {
        grid: v,
        width,
        height,
    }
}

fn number_of_flashes(input: &str, iterations: usize) -> usize {
    let mut floor: OceanFloor = parse_input(input);
    // println!("Initial State:");
    // println!("{}", floor);
    let mut flashes = 0;
    for n in 0..iterations {
        let number_flashed = floor.step();
        flashes += number_flashed;

        if number_flashed == floor.height * floor.width {
            println!("STOP! all octopi flashed at once at iteration {}", n);
            break;
        }
        // println!("State after {} iterations:", n + 1);
        // println!("{}", floor);
    }

    flashes
}

fn main() {
    const FULL: &str = include_str!("../input.txt");
    let number_of_iterations = 100;
    let num_flashes = number_of_flashes(FULL, number_of_iterations);
    println!(
        "Number of flashes after {} iterations: {}",
        number_of_iterations, num_flashes
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;
    const FULL: &str = include_str!("../input.txt");

    #[test]
    fn does_part_one_example() {
        let number_of_iterations = 100;
        let num_flashes = number_of_flashes(EXAMPLE, number_of_iterations);
        /*
            Wrong answer discovered:
            - 2011
        */
        assert_eq!(num_flashes, 1656);
    }

    #[test]
    fn does_part_one_full() {
        let number_of_iterations = 100;
        let num_flashes = number_of_flashes(FULL, number_of_iterations);
        assert_eq!(num_flashes, 1656);
    }

    #[test]
    fn does_part_two_full() {
        let number_of_iterations = 1_000;
        let num_flashes = number_of_flashes(FULL, number_of_iterations);
        assert_eq!(num_flashes, 6653);
        // should print out number of iterations once they fully synchronize (iteration 400)
    }

    #[test]
    fn read_cell_works() {
        let f = parse_input(EXAMPLE);
        assert_eq!(f.read_cell(&Coords(0, 0)), Some(&5));
        assert_eq!(f.read_cell(&Coords(5, 5)), Some(&2));
        assert_eq!(f.read_cell(&Coords(9, 9)), Some(&6));
    }

    #[test]
    fn get_mut_cell_works() {
        let mut f = parse_input(EXAMPLE);
        assert_eq!(f.get_mut_cell(&Coords(0, 0)), Some(&mut 5));
        assert_eq!(f.get_mut_cell(&Coords(5, 5)), Some(&mut 2));
        assert_eq!(f.get_mut_cell(&Coords(9, 9)), Some(&mut 6));
    }
}
