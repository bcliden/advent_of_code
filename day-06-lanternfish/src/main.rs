use std::{collections::{HashMap, VecDeque}, iter::FromIterator, ops::Deref};

struct LanternFish(u8);

impl LanternFish {
    fn new() -> Self {
        LanternFish(8)
    }

    fn from_input(s: &str) -> Self {
        Self(s.parse().unwrap())
    }

    fn tick(&mut self, swarm: &mut Vec<LanternFish>) {
        match self.0 {
            0 => {
                swarm.push(LanternFish::new());
                self.0 = 6;
            }
            _ => self.0 -= 1,
        }
    }
}

struct FishSwarm {
    fish: Vec<LanternFish>,
}

impl FishSwarm {
    fn new(fish: Vec<LanternFish>) -> Self {
        FishSwarm { fish }
    }

    fn simulate_day(&mut self) {
        let mut new_fish: Vec<LanternFish> = vec![];
        for f in self.fish.iter_mut() {
            f.tick(&mut new_fish);
        }
        // merge any new fish into the pack after the day
        self.fish.append(&mut new_fish);
    }

    fn count_fish(&self) -> usize {
        self.fish.len()
    }
}

/// So this was the really obvious brute force solution
/// and at 256 days will take a painful amount of time and memory to run
/// though it works fine for 80 days
fn part_1_solution(input: &str, days_to_sim: usize) -> usize {
    let fish_vec: Vec<_> = input
        .trim()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .map(|n| LanternFish(n))
        .collect();
    let mut swarm = FishSwarm::new(fish_vec);
    for _ in 0..days_to_sim {
        swarm.simulate_day()
    }
    swarm.count_fish()
}

/// algorithmically, this is much better. keep track of the NUMBER of fish,
/// not some weird idea of individual fish (???)
///
/// 1. We keep indices for each lifecyle phase (0-8)
/// 2. move groups to new places in hashmap accordingly (arithmetic)
/// 3. spawn new groups as needed
///
fn part_2_solution(input: &str, days_to_sim: usize) -> usize {
    let fish_vec: Vec<usize> = input
        .trim()
        .split(',')
        .filter_map(|s| s.parse().ok())
        // .map(|n| LanternFish(n))
        .collect();

    let mut fish_map: HashMap<usize, usize> = HashMap::new();
    for fish in fish_vec {
        let amount = fish_map.entry(fish).or_default();
        *amount += 1;
    }

    for _ in 0..days_to_sim {
        let map = fish_map.clone(); // clone, then rip that apart?
        for (number, count) in map.into_iter() {
            if number == 0 {
                // add new fish to next lifecycle
                let new_fishes = fish_map.entry(8).or_default();
                *new_fishes += count;

                // add reset fish to next lifecycle
                let reset_fish = fish_map.entry(6).or_default();
                *reset_fish += count;

                // finally, remove current fish count
                let current_fish = fish_map.entry(number).or_default();
                *current_fish -= count;
            } else {
                // otherwise, add all fish from current level to next lowest lifecycle
                let current_fish = fish_map.entry(number - 1).or_default();
                *current_fish += count;

                // finally, remove current fish count
                let current_fish = fish_map.entry(number).or_default();
                *current_fish -= count;
            }
        }
    }

    fish_map.values().sum()
}

/// this is a much nicer deque solution that 
/// takes advantage of VecDeque.rotate... 
/// though I probably could have done this with a bare array too.
fn part_2_solution_deque (input: &str, days_to_sim: usize) -> usize {
    let fish_vec: Vec<usize> = input
        .trim()
        .split(',')
        .filter_map(|s| s.parse().ok())
        // .map(|n| LanternFish(n))
        .collect();

    // get dequeue
    let mut fish_deque: VecDeque<usize> = VecDeque::from_iter([0usize; 9].iter().cloned());
    // load with fishies
    for f in fish_vec {
        let amount = fish_deque.get_mut(f).unwrap();
        *amount += 1;
    }

    for _ in 0..days_to_sim { 
        // get the fish to save (idx 0)
        let saved_fish = fish_deque.get(0).unwrap().clone();

        // rotate the whole thing left
        fish_deque.rotate_left(1);

        // finally, add the saved fish to idx6. new fish roll into idx8
        let amount = fish_deque.get_mut(6).unwrap();
        *amount += saved_fish;
    }

    fish_deque.into_iter().sum()
}

/// This is a much nicer solution.
/// Using the base array makes rotating and manipulation MUCH simpler.
/// 
/// I'm not sure that a HashMap has merits over an array in this case.
fn part_2_solution_array (input: &str, days_to_sim: usize) -> usize {
    let fish_vec: Vec<usize> = input
        .trim()
        .split(',')
        .filter_map(|s| s.parse().ok())
        // .map(|n| LanternFish(n))
        .collect();

    // make array and load with fishies
    let mut fish = [0 as usize; 9];
    for lifecycle in fish_vec {
        fish[lifecycle] += 1;
    }

    for _ in 0..days_to_sim { 
        // get the fish to save (idx 0)
        let saved_fish = fish[0];

        // rotate the whole thing left
        fish.rotate_left(1);

        // finally, add the saved fish to idx6. new fish roll into idx8
        fish[6] += saved_fish;
    }

    fish.iter().sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let number = part_2_solution(input, 80);
    println!("Total fish over 80 days: {}", number);

    // let number = part_1_solution(input, 80);
    // assert_eq!(number, 361169);

    let count = part_2_solution(input, 256);
    println!("Total fish over 256 days: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input.txt");
    const EXAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn runs_pt1_example() {
        let number = part_1_solution(EXAMPLE, 80);
        assert_eq!(number, 5934);
    }

    #[test]
    fn runs_pt1_full() {
        let number = part_1_solution(INPUT, 80);
        assert_eq!(number, 361169);
    }

    #[test]
    fn runs_pt2_example() {
        let number = part_2_solution(EXAMPLE, 256);
        assert_eq!(number, 26984457539);
    }

    #[test]
    fn runs_pt2_full() {
        let number = part_2_solution(INPUT, 256);
        assert_eq!(number, 1634946868992);
    }

    #[test]
    fn runs_pt2_full_deque() {
        let number = part_2_solution_deque(INPUT, 256);
        assert_eq!(number, 1634946868992);
    }

    #[test]
    fn runs_pt2_full_array() {
        let number = part_2_solution_array(INPUT, 256);
        assert_eq!(number, 1634946868992);
    }
}
