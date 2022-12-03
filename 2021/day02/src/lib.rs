use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day2_part1(reader: BufReader<File>) -> i32 {
    let mut position = (0, 0);

    for line in reader.lines() {
        let line = line.unwrap();
        let input = line.split(' ').collect::<Vec<&str>>();
        let distance: i32 = input[1].parse().unwrap();
        match input[0] {
            "forward" => position.0 += distance,
            "up" => position.1 -= distance,
            "down" => position.1 += distance,
            _ => {}
        }
    }

    position.0 * position.1
}

pub fn day2_part2(reader: BufReader<File>) -> i32 {
    let mut position = (0, 0);
    let mut aim = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let input = line.split(' ').collect::<Vec<&str>>();
        let value: i32 = input[1].parse().unwrap();
        match input[0] {
            "forward" => {
                position.0 += value;
                position.1 += aim * value;
            }
            "up" => aim -= value,
            "down" => aim += value,
            _ => {}
        }
    }

    position.0 * position.1
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use crate::{day2_part1, day2_part2};

    #[test]
    fn basic_example_part1() {
        let input_file = File::open("basic_example.txt").unwrap();
        let reader = BufReader::new(input_file);
        assert_eq!(day2_part1(reader), 150)
    }

    #[test]
    fn basic_example_part2() {
        let input_file = File::open("basic_example.txt").unwrap();
        let reader = BufReader::new(input_file);
        assert_eq!(day2_part2(reader), 900)
    }

    #[test]
    fn day1_part1() {
        let input_file = File::open("input.txt").unwrap();
        let reader = BufReader::new(input_file);
        assert_eq!(day2_part1(reader), 1648020)
    }

    #[test]
    fn day1_part2() {
        let input_file = File::open("input.txt").unwrap();
        let reader = BufReader::new(input_file);
        assert_eq!(day2_part2(reader), 1759818555)
    }
}
