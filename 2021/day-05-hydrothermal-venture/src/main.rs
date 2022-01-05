use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Pair(usize, usize);
impl<'a> FromIterator<&'a str> for Pair {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Pair(
            // wow, yuck!
            iter.next().unwrap().trim().parse().unwrap(),
            iter.next().unwrap().trim().parse().unwrap()
        )
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Horizontal,
    Vertical,
    DiagonalAsc, // always 45' angle
    DiagonalDesc, // always 45' angle
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinates(Pair, Pair);
impl Coordinates {
    fn direction(&self) -> Direction {
        let ((x1, y1), (x2, y2)) = self.to_parts();

        if x1 == x2 {
            Direction::Vertical
        } else if y1 == y2 {
            Direction::Horizontal
        } else if (x1 < x2 && y1 < y2) || (x1 > x2 && y1 > y2) {
            Direction::DiagonalAsc
        } else /* if (x1 < y2 && x2 < y1) || (x1 > y2 && y2 > y1) */ {
            Direction::DiagonalDesc
        }
    }

    fn to_parts(&self) -> ((usize, usize), (usize, usize)) {
        ((self.0 .0, self.0 .1), (self.1 .0, self.1 .1))
    }
}
impl FromIterator<Pair> for Coordinates {
    fn from_iter<T: IntoIterator<Item = Pair>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Coordinates(iter.next().unwrap(), iter.next().unwrap())
    }
}

struct Board {
    // NxN board
    side_width: usize,
    squares: Vec<usize>,
}
impl Board {
    fn new(side_width: usize) -> Self {
        Board {
            side_width,
            squares: vec![0; side_width * side_width],
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        // just sorta fake a 2d array
        (y * self.side_width) + x
    }

    fn mark_vent(&mut self, coord: Coordinates) {
        println!("marking: {:?}!", coord);
        let direction = coord.direction();
        match direction {
            Direction::Horizontal => {
                let ((x1, y), (x2, _)) = coord.to_parts();
                let mut start = x1.min(x2);
                let end = x1.max(x2);

                while start <= end {
                    let idx = self.get_index(start, y);
                    let num = self.squares.get_mut(idx).unwrap();
                    *num = *num + 1;
                    start += 1;
                }
            }
            Direction::Vertical => {
                let ((x, y1), (_, y2)) = coord.to_parts();
                let mut start = y1.min(y2);
                let end = y1.max(y2);

                while start <= end {
                    let idx = self.get_index(x, start);
                    let num = self.squares.get_mut(idx).unwrap();
                    *num = *num + 1;
                    start += 1;
                }
            }
            Direction::DiagonalAsc => {
                // bottom left to top right
                // x increasing, y increasing

                let ((x1, y1), (x2, y2)) = coord.to_parts();

                let mut x_start = x1.min(x2);
                let x_end = x1.max(x2);

                let mut y_start = y1.min(y2);
                let y_end = y1.max(y2);

                while x_start <= x_end && y_start <= y_end {
                    let idx = self.get_index(x_start, y_start);
                    let num = self.squares.get_mut(idx).unwrap();

                    *num = *num + 1;
                    x_start += 1;
                    y_start += 1;
                }
            }
            Direction::DiagonalDesc => {
                // top left to bottom right... same as above?
                // x increasing, y decreasing

                let ((x1, y1), (x2, y2)) = coord.to_parts();

                let mut x_start = x1.min(x2);
                let x_end = x1.max(x2);

                let mut y_start = y1.max(y2);
                let y_end = y1.min(y2);

                while x_start <= x_end && y_start >= y_end {
                    let idx = self.get_index(x_start, y_start);
                    let num = self.squares.get_mut(idx).unwrap();

                    *num = *num + 1;
                    x_start += 1;
                    y_start = y_start.saturating_sub(1);
                }
            }
        }
    }
}

fn do_part1(input: &str, board_dim: usize) -> usize {
    let coords: Vec<Coordinates> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pair| pair.split(',').collect::<Pair>())
                .collect::<Coordinates>()
        })
        .filter(|c|{
            let dir = c.direction();
            dir == Direction::Horizontal || dir == Direction::Vertical
        })
        .collect();

    let mut board = Board::new(board_dim);
    for coord in coords {
        board.mark_vent(coord);
    }
    board.squares.into_iter().filter(|n| *n > 1).count()
}

fn do_part2(input: &str, board_dim: usize) -> usize {
    let coords: Vec<Coordinates> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pair| pair.split(',').collect::<Pair>())
                .collect::<Coordinates>()
        })
        .collect();

    let mut board = Board::new(board_dim);
    for coord in coords {
        board.mark_vent(coord);
    }
    board.squares.into_iter().filter(|n| *n > 1).count()
}

fn main() {
    let s = include_str!("../input.txt");
    println!("Part 1: {:?}", do_part1(s, 1000));
    println!("Part 2: {:?}", do_part2(s, 1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2
"#;

    #[test]
    fn it_runs_pt1_example() {
        assert_eq!(5, do_part1(TEST_INPUT, 10))
    }

    #[test]
    fn it_runs_pt1_full() {
        let s = include_str!("../input.txt");
        assert_eq!(5585, do_part1(s, 1000))
    }

    #[test]
    fn it_runs_pt2_example() {
        assert_eq!(12, do_part2(TEST_INPUT, 10))
    }

    #[test]
    fn it_runs_pt2_full() {
        let s = include_str!("../input.txt");
        assert_eq!(17193, do_part2(s, 1000))
    }

    #[test]
    fn detects_diagonals() {
        let c = Coordinates(Pair(1, 1), Pair(3, 3));
        assert_eq!(c.direction(), Direction::DiagonalAsc);

        let c = Coordinates(Pair(1, 3), Pair(3, 1));
        assert_eq!(c.direction(), Direction::DiagonalDesc);
    }
}
