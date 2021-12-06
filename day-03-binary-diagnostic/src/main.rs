use std::collections::HashMap;

/// This is in no way fast or good, but it's correct

fn main() {
    let mut map: HashMap::<usize, Vec<char>> = HashMap::new();
    let str = include_str!("../input.txt");

    // initialize all vecs using the length of the first line
    let len = str.lines().next().unwrap().len();
    for i in 0..len {
        map.insert(i, vec![]);
    }

    // fill all chars into vecs
    for line in str.lines() {
        for (idx, char) in line.chars().enumerate() {
            map.get_mut(&idx).unwrap().push(char);
        } 
    }

    // get GAMMA rate (using most common chars)
    let mut gamma = String::with_capacity(len);
    for i in 0..len {
        let l = map.get(&i).unwrap();
        let zeroes = l.iter().filter(|c| **c == '0').count();
        let ones = l.iter().filter(|c| **c == '1').count();

        if ones > zeroes {
            gamma.insert(i, '1');
        } else {
            gamma.insert(i, '0');
        }
    }

    // get EPSILON rate (using least common chars)
    let mut epsilon = String::with_capacity(len);
    for i in 0..len {
        let l = map.get(&i).unwrap();
        let zeroes = l.iter().filter(|c| **c == '0').count();
        let ones = l.iter().filter(|c| **c == '1').count();

        if ones < zeroes { // <--- this is the ONLY difference from the gamma rate
            epsilon.insert(i, '1');
        } else {
            epsilon.insert(i, '0');
        }
    }

    let gamma_number = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_number = usize::from_str_radix(&epsilon, 2).unwrap();

    println!("gamma {}, epsilon {}", gamma, epsilon);
    println!("gamma {}, epsilon {}", gamma_number, epsilon_number);
    println!("multiplying... [{}]", gamma_number * epsilon_number);
}
