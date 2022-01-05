use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
struct Square {
    value: usize,
    checked: bool,
}
impl Square {
    fn new(value: usize) -> Self {
        Square {
            value,
            checked: false,
        }
    }

    fn check(&mut self) {
        self.checked = true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
struct Board {
    // 5x5 board
    board: Vec<Square>,
}
impl Board {
    fn new(values: impl Iterator<Item = usize>) -> Self {
        let squares: Vec<_> = values.map(|num| Square::new(num)).collect();

        assert_eq!(squares.len(), 25); // a proper board is 5x5

        Board { board: squares }
    }

    fn from_input(input: &[&str]) -> Board {
        Board::new(Board::from_str_to_vec(input).into_iter())
    }

    fn mark_number(&mut self, n: usize) {
        if let Some(sq) = self.board.iter_mut().find(|sq| sq.value == n) {
            sq.check()
        }
    }

    fn get_index(row: usize, col: usize) -> usize {
        (row * 5) + col
    }

    fn from_str_to_vec(strs: &[&str]) -> Vec<usize> {
        strs.iter()
            .map(|line| {
                line.split_ascii_whitespace()
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect::<Vec<usize>>()
            })
            .fold(vec![], |mut acc, mut next| {
                acc.append(&mut next);
                acc
            })
    }

    fn is_winner(&self) -> Option<&Self> {
        if vec![
            // across 5 ways
            self.check_row(0),
            self.check_row(1),
            self.check_row(2),
            self.check_row(3),
            self.check_row(4),
            // down 5 ways
            self.check_col(0),
            self.check_col(1),
            self.check_col(2),
            self.check_col(3),
            self.check_col(4),
            // diagonal twice
            // self.check_diagonals(), // NOT part of the AC :(
        ]
        .iter()
        .any(|b| *b)
        {
            Some(&self)
        } else {
            None
        }
    }

    fn check_some<F>(&self, mut translate_idx: F) -> bool
    where
        F: FnMut(usize) -> usize,
    {
        (0..5usize)
            .map(|n| translate_idx(n))
            .map(|n| self.board.get(n).unwrap())
            .all(|sq| sq.checked)
    }

    fn check_row(&self, row: usize) -> bool {
        self.check_some(|n| Board::get_index(row, n))
    }

    fn check_col(&self, col: usize) -> bool {
        self.check_some(|n| Board::get_index(n, col))
    }

    fn check_diagonals(&self) -> bool {
        let mut rl_counter: usize = 5;
        let r_to_l = self.check_some(|row| {
            rl_counter -= 1;
            let idx = Board::get_index(row, rl_counter);
            idx
        });

        let mut lr_counter: usize = 0;
        let l_to_r = self.check_some(|row| {
            let idx = Board::get_index(row, lr_counter);
            lr_counter += 1;
            idx
        });

        l_to_r || r_to_l
    }
}

struct WinningPair {
    board: Board,
    last_number: usize,
}

impl From<RankedBoard> for WinningPair {
    fn from(ranked_board: RankedBoard) -> Self {
        WinningPair::from(&ranked_board)
    }
}

impl From<&RankedBoard> for WinningPair {
    fn from(ranked_board: &RankedBoard) -> Self {
        WinningPair {
            board: ranked_board.board.clone(),
            last_number: ranked_board.winning_number,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
struct RankedBoard {
    board: Board,
    turns_to_win: usize,
    winning_number: usize,
}

impl RankedBoard {
    fn new(mut board: Board, nums: Vec<usize>) -> Option<RankedBoard> {
        // OK, returning Option<Self> is a pretty weird decision here.. should be parse() or rank() instead of new()
        for (idx, n) in nums.into_iter().enumerate() {
            board.mark_number(n);
            if let Some(_) = board.is_winner() {
                return Some(RankedBoard {
                    board,
                    turns_to_win: idx,
                    winning_number: n,
                });
            }
        }
        None
    }
}

impl Ord for RankedBoard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.turns_to_win.cmp(&other.turns_to_win)
    }
}

#[derive(Debug)]
struct BoardSet {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}
impl BoardSet {
    fn from_input(input: &str) -> Self {
        let mut lines: Vec<&str> = input.lines().collect();
        let numbers: Vec<usize> = lines[0].split(',').filter_map(|s| s.parse().ok()).collect();
        // skip 1 (empty)
        let contains_boards = lines.split_off(2);
        let chunked = contains_boards.chunks(6);
        let boards: Vec<Board> = chunked.map(|chunk| Board::from_input(chunk)).collect();

        BoardSet { numbers, boards }
    }

    fn run_numbers(&mut self) -> Option<WinningPair> {
        let numbers = self.numbers.clone();
        let mut called_numbers = vec![];
        for n in numbers {
            self.call_number(n);
            called_numbers.push(n);

            if let Some(winner) = self.check_winners() {
                // return Some(winner.clone());
                return Some(WinningPair {
                    board: winner.clone(),
                    last_number: n,
                });
            }
        }
        None
    }

    fn rank_all_boards(&self) -> Vec<RankedBoard> {
        self.boards
            .iter()
            .filter_map(|b| RankedBoard::new(b.clone(), self.numbers.clone()))
            .collect()
    }

    fn call_number(&mut self, n: usize) {
        for b in self.boards.iter_mut() {
            b.mark_number(n);
        }
    }

    fn check_winners(&self) -> Option<&Board> {
        for board in self.boards.iter() {
            let opt = board.is_winner();
            if opt.is_some() {
                return opt;
            }
        }
        None
    }
}

fn calc_score(winner: &WinningPair) -> usize {
    // sum of all unmarked numbers on the board
    // multiplied by the number that was just called
    let unmarked = winner.board.board.iter().filter(|sq| !sq.checked);
    let sum = unmarked.fold(0, |acc, next| acc + next.value);
    sum * winner.last_number
}

/*

    Part 1 strategy (obvious one)
    1. Building all boards
    2. iterating through numbers, scoring boards all together
    3. checking for a winner

    Part 2 strategy (new hotness)
    1. Build all boards
    2. Apply drawn numbers to each board individually, marking _how many_ steps it took to win
    3. picking the set.min() for fastest (or set.max() for slowest)

*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = include_str!("../input.txt");

    // Part 1
    let mut bs = BoardSet::from_input(s);
    let result = bs.run_numbers();
    match result {
        Some(_) => println!("you won!"),
        None => return Err("Hm... no board won".into()),
    };
    println!("{}", calc_score(&result.unwrap()));

    // Part 2: new approach!
    let bs = BoardSet::from_input(s);
    let ranked_boards = bs.rank_all_boards();
    let longest = ranked_boards.iter().max().unwrap();
    let score = calc_score(&WinningPair {
        board: longest.board.clone(),
        last_number: longest.winning_number,
    });
    println!("Winner! (slowest) score: {}", score);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7"#;

    #[test]
    fn does_the_thing() {
        let mut set = BoardSet::from_input(INPUT);
        let winner = set.run_numbers();
        assert!(winner.is_some());
        assert_eq!(4512, calc_score(&winner.unwrap()));
    }

    #[test]
    fn runs_the_real_input() {
        let s = include_str!("../input.txt");
        let mut set = BoardSet::from_input(s);
        let winner = set.run_numbers();
        assert!(winner.is_some());
        assert_eq!(41668, calc_score(&winner.unwrap()));
    }

    #[test]
    fn finds_the_longest_running() {
        let bs = BoardSet::from_input(INPUT);
        let ranked_boards = bs.rank_all_boards();
        let longest = ranked_boards.iter().max().unwrap();
        let score = calc_score(&longest.into());
        assert_eq!(score, 1924);
    }

    #[test]
    fn finds_the_longest_running_real_input() {
        let s = include_str!("../input.txt");
        let bs = BoardSet::from_input(s);
        let ranked_boards = bs.rank_all_boards();
        let longest = ranked_boards.iter().max().unwrap();
        let score = calc_score(&longest.into());
        assert_eq!(score, 10478);
    }

    const SINGLE_BOARD: &str = r#"     3 15  0  2 22
    9 18 13 17  5
   19  8  7 25 23
   20 11 10 24  4
   14 21 16 12  6"#;

    #[test]
    fn check_diags_works() {
        // check (0, 0) -> (4, 4) diagonal
        let mut b = Board::from_input(SINGLE_BOARD.lines().collect::<Vec<&str>>().as_slice());
        b.mark_number(3);
        b.mark_number(18);
        b.mark_number(7);
        b.mark_number(24);
        b.mark_number(6);
        let ret = b.check_diagonals();
        assert!(ret);

        // check (0, 4) -> (4, 0) diagonal
        let mut b = Board::from_input(SINGLE_BOARD.lines().collect::<Vec<&str>>().as_slice());
        b.mark_number(22);
        b.mark_number(17);
        b.mark_number(7);
        b.mark_number(11);
        b.mark_number(14);
        let ret = b.check_diagonals();
        assert!(ret);
    }

    #[test]
    fn check_rows_works() {
        let mut b = Board::from_input(SINGLE_BOARD.lines().collect::<Vec<&str>>().as_slice());
        b.mark_number(3);
        b.mark_number(15);
        b.mark_number(0);
        b.mark_number(2);
        b.mark_number(22);
        let ret = b.check_row(0);
        assert!(ret);
    }

    #[test]
    fn check_cols_works() {
        let mut b = Board::from_input(SINGLE_BOARD.lines().collect::<Vec<&str>>().as_slice());
        b.mark_number(3);
        b.mark_number(9);
        b.mark_number(19);
        b.mark_number(20);
        b.mark_number(14);
        let ret = b.check_col(0);
        assert!(ret);
    }

    #[test]
    fn diagonals_dont_count() {
        let mut winning_board =
            Board::from_input(&SINGLE_BOARD.lines().collect::<Vec<&str>>().as_slice());
        winning_board.mark_number(3);
        winning_board.mark_number(18);
        winning_board.mark_number(7);
        winning_board.mark_number(24);
        winning_board.mark_number(6);
        // this board SHOULD win on a diagonal
        let ret = winning_board.is_winner();
        assert!(ret.is_none());
    }

    #[test]
    fn check_any() {
        let mut winning_board =
            Board::from_input(&SINGLE_BOARD.lines().collect::<Vec<&str>>().as_slice());
        // 9 18 13 17  5
        winning_board.mark_number(9);
        winning_board.mark_number(18);
        winning_board.mark_number(13);
        winning_board.mark_number(17);
        winning_board.mark_number(5);
        // this board SHOULD win on a the second row
        let ret = winning_board.is_winner();
        assert!(ret.is_some());

        let losing_board =
            Board::from_input(&SINGLE_BOARD.lines().collect::<Vec<&str>>().as_slice());
        let ret = losing_board.is_winner();
        assert!(ret.is_none());
    }
}
