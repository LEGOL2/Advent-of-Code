use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn find_len_of_numbers(numbers: &Vec<i32>) -> usize {
    let mut max = numbers.iter().max().unwrap().clone();
    let mut len = 0;
    while max > 0 {
        max = max >> 1;
        len += 1;
    }

    len
}

fn get_num_of_zeros_and_ones(numbers: &Vec<i32>, length: usize) -> (Vec<i32>, Vec<i32>) {
    let mut num_of_zeros = vec![0; length];
    let mut num_of_ones = vec![0; length];

    for number in numbers.iter() {
        for i in (0..length).rev() {
            let bit = number & (1 << i);
            match bit {
                0 => num_of_zeros[i] += 1,
                _ => num_of_ones[i] += 1,
            }
        }
    }

    (num_of_zeros, num_of_ones)
}

pub fn day3_part1(reader: BufReader<File>) -> i32 {
    let numbers: Vec<i32> = reader
        .lines()
        .map(|n| i32::from_str_radix(&n.unwrap(), 2).unwrap())
        .collect();
    let length = find_len_of_numbers(&numbers);
    let (num_of_zeros, num_of_ones) = get_num_of_zeros_and_ones(&numbers, length);

    let mut gamma = 0;
    let mut epsilon = 0;
    for i in (0..length).rev() {
        if num_of_ones[i] > num_of_zeros[i] {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    gamma * epsilon
}

pub fn day3_part2(reader: BufReader<File>) -> i32 {
    let mut numbers: Vec<i32> = reader
        .lines()
        .map(|n| i32::from_str_radix(&n.unwrap(), 2).unwrap())
        .collect();
    let mut second_numbers = numbers.clone();
    let length = find_len_of_numbers(&numbers);

    let a = run_diagnostics(length, &mut numbers, true);
    let b = run_diagnostics(length, &mut &mut second_numbers, false);

    a * b
}

fn run_diagnostics(length: usize, numbers: &mut Vec<i32>, oxygen_generator_rating: bool) -> i32 {
    for i in (0..length).rev() {
        if numbers.len() == 1 {
            break;
        }

        let (num_of_zeros, num_of_ones) = get_num_of_zeros_and_ones(&numbers, length);

        if oxygen_generator_rating {
            if num_of_zeros[i] > num_of_ones[i] {
                numbers.retain(|n| n & (1 << i) == 0);
            } else if num_of_ones[i] > num_of_zeros[i] {
                numbers.retain(|n| n & (1 << i) != 0);
            } else {
                numbers.retain(|n| n & (1 << i) != 0);
            }
        } else {
            if num_of_zeros[i] > num_of_ones[i] {
                numbers.retain(|n| n & (1 << i) != 0);
            } else if num_of_ones[i] > num_of_zeros[i] {
                numbers.retain(|n| n & (1 << i) == 0);
            } else {
                numbers.retain(|n| n & (1 << i) == 0);
            }
        }
    }

    numbers[0]
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use crate::{day3_part1, day3_part2, find_len_of_numbers};

    #[test]
    fn basic_example_part1() {
        let file = File::open("basic_example.txt").expect("File not found");
        let reader = BufReader::new(file);
        assert_eq!(day3_part1(reader), 198);
    }

    #[test]
    fn basic_example_part2() {
        let file = File::open("basic_example.txt").expect("File not found");
        let reader = BufReader::new(file);
        assert_eq!(day3_part2(reader), 230);
    }

    #[test]
    fn day3_part1_test() {
        let file = File::open("input.txt").expect("File not found");
        let reader = BufReader::new(file);
        assert_eq!(day3_part1(reader), 3882564);
    }

    #[test]
    fn day3_part2_test() {
        let file = File::open("input.txt").expect("File not found");
        let reader = BufReader::new(file);
        assert_eq!(day3_part2(reader), 3385170);
    }

    #[test]
    fn length_of_input() {
        let file = File::open("input.txt").expect("File not found");
        let reader = BufReader::new(file);
        let numbers: Vec<i32> = reader
            .lines()
            .map(|n| i32::from_str_radix(&n.unwrap(), 2).unwrap())
            .collect();
        let len = find_len_of_numbers(&numbers);
        assert_eq!(len, 12);
    }

    #[test]
    fn length_of_basic_example_input() {
        let file = File::open("basic_example.txt").expect("File not found");
        let reader = BufReader::new(file);
        let numbers: Vec<i32> = reader
            .lines()
            .map(|n| i32::from_str_radix(&n.unwrap(), 2).unwrap())
            .collect();
        let len = find_len_of_numbers(&numbers);
        assert_eq!(len, 5);
    }
}
