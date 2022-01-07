use std::{fmt::Debug, ops::AddAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Vec2 {
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Open,
    Tree,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Open
    }
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Open => '.',
            Self::Tree => '#',
        };
        write!(f, "{}", c)
    }
}

struct Map {
    size: Vec2,
    tiles: Vec<Tile>,
}

impl Map {
    fn new(size: Vec2) -> Self {
        let num_tiles = size.x * size.y;
        Self {
            size,
            tiles: (0..num_tiles)
                .into_iter()
                .map(|_| Default::default())
                .collect(),
        }
    }

    fn set(&mut self, pos: Vec2, tile: Tile) {
        if let Some(index) = self.index(pos) {
            self.tiles[index] = tile
        }
    }

    fn get(&self, pos: Vec2) -> Tile {
        self.index(pos).map(|i| self.tiles[i]).unwrap_or_default()
    }

    fn normalize_pos(&self, pos: Vec2) -> Option<Vec2> {
        if pos.y < 0 || pos.y >= self.size.y {
            // return None if off the top or bottom
            None
        } else {
            // i should revisit how/why this works with negatives, but it's passing nicely
            let x = pos.x % self.size.x;
            // wrap around positions for the left side
            let x = if x < 0 { self.size.x + x } else { x };
            Some((x, pos.y).into())
        }
    }

    fn index(&self, pos: Vec2) -> Option<usize> {
        self.normalize_pos(pos)
            .map(|pos| (pos.x + pos.y * self.size.x) as _)
    }

    fn parse(input: &[u8]) -> Self {
        let mut columns = 0;
        let mut rows = 1;
        for &c in input.iter() {
            if c == b'\n' {
                rows += 1;
                columns = 0;
            } else {
                columns += 1;
            }
        }

        let mut iter = input.iter().copied();
        let mut map = Self::new((columns, rows).into());
        for row in 0..map.size.y {
            for col in 0..map.size.x {
                let tile = match iter.next() {
                    Some(b'.') => Tile::Open,
                    Some(b'#') => Tile::Tree,
                    c => panic!("Expected '.' or '#' but got {:?}", c),
                };
                map.set((col, row).into(), tile);
            }
            iter.next();
        }
        map
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.size.y {
            for col in 0..self.size.x {
                write!(f, "{:?}", self.get((col, row).into()))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part_one_main(bytes: &[u8]) -> usize {
    let map = Map::parse(bytes);
    let itinerary = (0..map.size.y).into_iter().map(|y| Vec2::from((y * 3, y)));
    itinerary.filter(|&pos| map.get(pos) == Tile::Tree).count()
}

fn part_two_main(bytes: &[u8]) -> usize {
    let map = Map::parse(bytes);

    // from the AoC prompt
    let deltas: &[Vec2] = &[
        (1, 1).into(),
        (3, 1).into(),
        (5, 1).into(),
        (7, 1).into(),
        (1, 2).into(),
    ];

    deltas
        .iter()
        .copied()
        .map(|delta| generate_itinerary(&map, delta))
        .map(|itinerary| {
            itinerary
                .into_iter()
                .filter(|&pos| map.get(pos) == Tile::Tree)
                .count()
        })
        .product::<usize>()
}

fn generate_itinerary(map: &Map, delta: Vec2) -> Vec<Vec2> {
    let mut pos = Vec2::from((0, 0));
    let mut res: Vec<_> = Default::default();

    while map.normalize_pos(pos).is_some() {
        res.push(pos);
        pos += delta;
    }

    res
}

fn main() {
    let bytes = include_bytes!("input.txt");

    // part one
    let count = part_one_main(bytes);
    println!("we encountered {} trees", count);

    // part two
    let count = part_two_main(bytes);
    println!("we encountered {} trees", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;

    #[test]
    fn part_one_example() {
        let count = part_one_main(EXAMPLE.as_bytes());
        assert_eq!(7, count);
    }

    #[test]
    fn part_one_full() {
        let count = part_one_main(include_bytes!("input.txt"));
        assert_eq!(167, count);
    }

    #[test]
    fn part_two_example() {
        let count = part_two_main(EXAMPLE.as_bytes());
        assert_eq!(336, count);
    }

    #[test]
    fn part_two_full() {
        let count = part_two_main(include_bytes!("input.txt"));
        assert_eq!(736527114, count);
    }

    #[test]
    fn test_tuple() {
        let v: Vec2 = (5, 8).into();
        assert_eq!(v.x, 5);
        assert_eq!(v.y, 8);
    }

    #[test]
    fn test_normalize_pos() {
        let m = Map::new((2, 2).into());
        assert_eq!(m.normalize_pos((0, 0).into()), Some((0, 0).into()));
        assert_eq!(m.normalize_pos((1, 0).into()), Some((1, 0).into()));
        assert_eq!(m.normalize_pos((2, 0).into()), Some((0, 0).into()));
        assert_eq!(m.normalize_pos((-1, 0).into()), Some((1, 0).into()));
        assert_eq!(m.normalize_pos((-2, 0).into()), Some((0, 0).into()));
        assert_eq!(m.normalize_pos((0, -1).into()), None);
        assert_eq!(m.normalize_pos((0, 2).into()), None);
    }

    #[test]
    fn test_index() {
        let m = Map::new((3, 5).into());
        assert_eq!(m.index((0, 0).into()), Some(0));
        assert_eq!(m.index((2, 0).into()), Some(2));
        assert_eq!(m.index((0, 1).into()), Some(3));
        assert_eq!(m.index((2, 1).into()), Some(5));
    }

    #[test]
    fn test_generate_itinerary() {
        assert_eq!(
            &generate_itinerary(&Map::new((5, 5).into()), (1, 1).into()),
            &[
                (0, 0).into(),
                (1, 1).into(),
                (2, 2).into(),
                (3, 3).into(),
                (4, 4).into(),
            ],
            "right 1 down 1, 5x5 map"
        );

        assert_eq!(
            &generate_itinerary(&Map::new((5, 5).into()), (3, 1).into()),
            &[
                (0, 0).into(),
                (3, 1).into(),
                (6, 2).into(),
                (9, 3).into(),
                (12, 4).into(),
            ],
            "right 3 down 1, 5x5 map"
        );

        assert_eq!(
            &generate_itinerary(&Map::new((5, 5).into()), (2, 2).into()),
            &[(0, 0).into(), (2, 2).into(), (4, 4).into(),],
            "right 2 down 2, 5x5 map"
        );
        assert_eq!(
            &generate_itinerary(&Map::new((9, 9).into()), (2, 5).into()),
            &[(0, 0).into(), (2, 5).into(),],
            "right 2 down 5, 9x9 map"
        )
    }
}
