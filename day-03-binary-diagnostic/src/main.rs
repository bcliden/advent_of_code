use std::{collections::HashMap, iter::FromIterator, ops::Not};

#[derive(Clone, Copy)]
struct BinaryCount {
    zero: usize,
    one: usize,
}

impl BinaryCount {
    fn new() -> Self {
        BinaryCount { zero: 0, one: 0 }
    }
}

/// This is nice on pt.1, but very wasteful the way I use it in pt.2
fn process_input<'a, T>(number_width: usize, input: T) -> HashMap<usize, BinaryCount>
where
    T: Iterator<Item = &'a str>,
{
    let mut map: HashMap<usize, BinaryCount> = HashMap::new();

    // initialize all BinaryCounts using the length of the first line
    for i in 0..number_width {
        map.insert(i, BinaryCount::new());
    }

    // fill all chars into vecs
    for line in input {
        for (idx, ch) in line.chars().enumerate() {
            if ch == '0' {
                // this is a little ugly, but not so bad
                map.get_mut(&idx).unwrap().zero += 1;
            } else {
                map.get_mut(&idx).unwrap().one += 1;
            }
        }
    }

    map
}

struct PowerConsumption {
    gamma: usize,
    epsilon: usize,
}

fn calculate_power_consumption(input: &str) -> PowerConsumption {
    let number_width = input.lines().next().unwrap().len();

    let map = process_input(number_width, input.lines());

    // get GAMMA rate (using most common chars)
    let mut gamma = String::with_capacity(number_width);
    for i in 0..number_width {
        let l = map.get(&i).unwrap();
        if l.one > l.zero {
            gamma.insert(i, '1');
        } else {
            gamma.insert(i, '0');
        }
    }

    // let usize_width = std::mem::size_of::<usize>() * 4; /* from bytes to bits */
    let gamma_number = usize::from_str_radix(&gamma, 2).unwrap();

    /* How does this work? What is this doing?

         usize::MAX is <all 1s>:     1111 1111
            shifting by number_width will allow us to mask out any non_original bits
         mask : usize::MAX << 4 is:  1111 0000

         THEN, we flip all the bits with usize.not(), and mask out any non-original bits.
         start (usize):  0000 1010
         usize::MAX      1111 1111
         mask:           1111 0000 (usize::MAX << 4)
         not'd:          1111 0101
         not XOR mask    0000 0101 (all mutual ones are zeroed out)
    */
    let epsilon_masked = gamma_number.not() ^ (usize::MAX << number_width);

    PowerConsumption {
        gamma: gamma_number,
        epsilon: epsilon_masked,
    }
}

struct LifeSupportRating {
    co2_scrubber: usize,
    oxygen_generator: usize,
}

fn sieve<'a, F>(number_width: usize, input: &'a str, filter_func: F) -> &'a str
where
    F: Fn(usize, usize) -> char,
{
    let mut filtering_vec: Vec<&str> = Vec::from_iter(input.trim().lines()); // will be slowly narrowed down to our number
    let mut idx: usize = 0; // track WHICH digit we're on
    while filtering_vec.len() != 1 {
        // reprocess the vec into a map count... very wasteful. We really only need counts PER idx, not whole numbers
        let map = process_input(number_width, filtering_vec.iter().cloned());
        let bc = map.get(&idx).unwrap();
        let digit_to_keep = filter_func(bc.zero, bc.one); // use closure to determine what char we retain

        // finally, do the filter
        filtering_vec.retain(|bin_string| bin_string.chars().nth(idx).unwrap() == digit_to_keep);
        idx += 1;
    }
    assert_eq!(filtering_vec.len(), 1);
    filtering_vec.get(0).unwrap()
}

fn calculate_life_support_rating(input: &str) -> LifeSupportRating {
    let number_width = input.lines().next().unwrap().len();

    let oxygen_generator = sieve(number_width, input, |zero, one| {
        if one > zero || one == zero { // MOST common, favoring ones in a tie
            '1'
        } else {
            '0'
        }
    });
    let co2_scrubber = sieve(number_width, input, |zero, one| {
        if zero < one || zero == one { // LEAST common, favoring zeroes in a tie
            '0'
        } else {
            '1'
        }
    });

    LifeSupportRating {
        co2_scrubber: usize::from_str_radix(&co2_scrubber, 2).unwrap(),
        oxygen_generator: usize::from_str_radix(&oxygen_generator, 2).unwrap(),
    }
}

fn main() {
    let str = include_str!("../input.txt");

    let pc = calculate_power_consumption(str);
    println!("gamma {}, epsilon_masked {}", pc.gamma, pc.epsilon);
    println!("multiplying... > {} <", pc.gamma * pc.epsilon);

    let ls = calculate_life_support_rating(str);
    println!("co2 {}, oxygen {}", ls.co2_scrubber, ls.oxygen_generator);
    println!(
        "multiplying... > {} <",
        ls.co2_scrubber * ls.oxygen_generator
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_the_right_power_consumption() {
        let str = include_str!("../input.txt");
        let pc = calculate_power_consumption(&str);
        assert_eq!(pc.gamma, 1143);
        assert_eq!(pc.epsilon, 2952);
        assert_eq!(pc.gamma * pc.epsilon, 3374136)
    }

    #[test]
    fn it_gets_the_right_life_support_rating() {
        let str = include_str!("../input.txt");
        let ls = calculate_life_support_rating(str);
        /*
            Wrong answers discovered:
            - 8737936
            - 5391684
        */
        assert_eq!(ls.oxygen_generator, 1909);
        assert_eq!(ls.co2_scrubber, 2322);
        assert_eq!(ls.oxygen_generator * ls.co2_scrubber, 4432698)
    }
}
