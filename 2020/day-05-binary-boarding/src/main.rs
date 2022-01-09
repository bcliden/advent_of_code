#![allow(dead_code)]

mod part_one {
    use bitvec::prelude::*;

    #[derive(Default, Debug, PartialEq)]
    struct Seat {
        row: u8,
        col: u8,
    }

    impl Seat {
        const ROW_BITS: usize = 7;
        const COL_BITS: usize = 3;

        fn parse(input: &str) -> Self {
            let bytes = input.as_bytes();
            let mut res: Seat = Default::default();

            {
                // operate on res.row 👇
                let row = BitSlice::<Msb0, _>::from_element_mut(&mut res.row);
                // for each F or B elem...
                for (i, &b) in bytes[0..Self::ROW_BITS].iter().enumerate() {
                    // set the corresponding bit, positions 0 - 6
                    row.set(
                        (8 - Self::ROW_BITS) + i,
                        match b {
                            b'F' => false,
                            b'B' => true,
                            _ => panic!("unexpected row letter: {}", b as char),
                        },
                    );
                }
            }
            {
                // operate on res.col 👇
                let col = BitSlice::<Msb0, _>::from_element_mut(&mut res.col);
                for (i, &b) in bytes[Self::ROW_BITS..][..Self::COL_BITS].iter().enumerate() {
                    col.set(
                        (8 - Self::COL_BITS) + i,
                        match b {
                            b'L' => false,
                            b'R' => true,
                            _ => panic!("unexpected col letter: {}", b as char),
                        },
                    );
                }
            }
            res
        }

        fn id(&self) -> u64 {
            // or do a confusing bit shift like so:
            // ((self.row as u64) << Self::COL_BITS) + (self.col as u64)
            // shifting 3 bits is the same result as multiplying by 8

            // but here's the simple solution:
            ((self.row as u64) * 8) + (self.col as u64)
        }
    }

    pub fn calc(s: &str) -> Option<u64> {
        s.lines().map(Seat::parse).map(|seat| seat.id()).max()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn part_one_full() {
            assert_eq!(calc(super::super::FULL_INPUT), Some(965))
        }

        #[test]
        fn test_parse() {
            // this is just two successive integers...
            // FBFBBFF == 0101100 == 44
            // RLR == 101 == 5
            let input = "FBFBBFFRLR";
            let seat = Seat::parse(input);
            assert_eq!(seat, Seat { row: 44, col: 5 });
        }

        #[test]
        fn test_id() {
            let seat = Seat { row: 102, col: 4 };
            assert_eq!(seat.id(), 820);
        }

        #[test]
        fn test_id_macro() {
            macro_rules! validate {
                ($input: expr, $row: expr, $col: expr, $id: expr) => {
                    let seat = Seat::parse($input);
                    assert_eq!(
                        seat,
                        Seat {
                            row: $row,
                            col: $col
                        }
                    );
                    assert_eq!(seat.id(), $id);
                };
            }

            // from the AoC examples
            validate!("BFFFBBFRRR", 70, 7, 567);
            validate!("FFFBBBFRRR", 14, 7, 119);
            validate!("BBFFBBFRLL", 102, 4, 820);
        }
    }
}

mod part_two {
    /*
        Combine the row + col values in a way that we just get an ID.
        Since the ID is multiplied by 8 (bitshift << 3),

        ticket: FBFBBFFRLR
        this is just two successive integers...
        FBFBBFF == 0101100 == 44
        RLR == 101 == 5

        and 44 << 3 == FBFBBFF000
                              ^^^
        and 0 + 5   == 0000000RLR
                              ^^^

        44 * 8 + 5 == FBFBBFFRLR !
        just convert them to binary *in place*!
        pretty wily!
    */

    use bitvec::prelude::*;

    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
    struct Seat(u16);

    impl Seat {
        fn parse(input: &str) -> Self {
            let mut res: Self = Default::default();

            // note the reversed Bit order... least to greatest
            let bits = BitSlice::<Lsb0, _>::from_element_mut(&mut res.0);
            // then iterate backwards so the indices are nice and neat!
            for (i, &b) in input.as_bytes().iter().rev().enumerate() {
                // rlrfbbfbf (reverse order!)
                bits.set(
                    i,
                    match b {
                        b'F' | b'L' => false, // low bits
                        b'B' | b'R' => true,  // high bits
                        _ => panic!("unexpected letter: {}", b as char),
                    },
                )
            }
            res
        }
    }

    fn calc(s: &str) -> Option<Seat> {
        let mut ids: Vec<_> = s.lines().map(Seat::parse).collect();
        ids.sort();

        let mut last_id: Option<Seat> = None;
        for id in ids {
            if let Some(last_id) = last_id {
                // calculate the gap between the last seat and current seat
                let gap = id.0 - last_id.0;
                // if it's greater than one, there's a hole for us to sit in :)
                if gap > 1 {
                    return Some(Seat(last_id.0 + 1));
                }
            }
            last_id = Some(id);
        }
        None
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn part_two_full() {
            assert_eq!(calc(super::super::FULL_INPUT), Some(Seat(524)))
        }

        #[test]
        fn test_parse() {
            let input = "FBFBBFFRLR";
            assert_eq!(Seat::parse(input), Seat(357));
        }

        #[test]
        fn test_seat_id() {
            assert_eq!(Seat::parse("BFFFBBFRRR"), Seat(567));
            assert_eq!(Seat::parse("FFFBBBFRRR"), Seat(119));
            assert_eq!(Seat::parse("BBFFBBFRLL"), Seat(820));
        }
    }
}

fn main() {
    println!("Hello, world! (see tests for actual solutions)");
}

const FULL_INPUT: &str = include_str!("input.txt");
