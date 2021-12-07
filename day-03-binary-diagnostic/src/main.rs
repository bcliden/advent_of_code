use std::{collections::HashMap, ops::Not};

struct PowerConsumption {
    gamma: usize,
    epsilon: usize
}

fn calculate_power_consumption(input: &str) -> PowerConsumption {
    let mut map: HashMap<usize, Vec<char>> = HashMap::new();

    // initialize all vecs using the length of the first line
    let number_width = input.lines().next().unwrap().len();
    for i in 0..number_width {
        map.insert(i, vec![]);
    }

    // fill all chars into vecs
    for line in input.lines() {
        for (idx, char) in line.chars().enumerate() {
            map.get_mut(&idx).unwrap().push(char);
        }
    }

    // get GAMMA rate (using most common chars)
    let mut gamma = String::with_capacity(number_width);
    for i in 0..number_width {
        let l = map.get(&i).unwrap();
        let zeroes = l.iter().filter(|c| **c == '0').count();
        let ones = l.len() - zeroes;

        if ones > zeroes {
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
        epsilon: epsilon_masked
    }
}

struct LifeSupportRating {
    co2_scrubber: usize,
    oxygen_generator: usize
}

fn calculate_life_support_rating(input: &str) -> LifeSupportRating {
    todo!()
}

fn main() {
    let str = include_str!("../input.txt");

    let pc = calculate_power_consumption(&str);
    println!("gamma {}, epsilon_masked {}", pc.gamma, pc.epsilon);
    println!("multiplying... > {} <", pc.gamma * pc.epsilon);

    let ls = calculate_life_support_rating(&str);
    println!("co2 {}, oxygen {}", ls.co2_scrubber, ls.oxygen_generator);
    println!("multiplying... > {} <", ls.co2_scrubber * ls.co2_scrubber);
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
        todo!();
    }
}
