use std::{fs::File, io::{BufReader, BufRead}};

pub fn find_number_of_easy_numbers_in_output(output_digits: &Vec<String>) -> usize {
    let mut counter = 0;

    for digit in output_digits.iter() {
        match digit.len() {
            2 | 3 | 4 | 7 => counter += 1,
            _ => (),
        }
    }

    counter
}

pub fn read_input(filename: &str) -> (Vec<String>, Vec<String>) {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines()
        .map(|l| l.expect("Could not parse the line."))
        .collect();
    
    let mut signal_patterns: Vec<String> = Vec::new();
    let mut output_digits: Vec<String> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split('|').collect();
        signal_patterns.extend(parts[0].strip_suffix(' ').unwrap().split(' ').map(|s| s.to_owned()));
        output_digits.extend(parts[1].strip_prefix(' ').unwrap().split(' ').map(|s| s.to_owned()));
    }

    (signal_patterns, output_digits)
}

#[cfg(test)]
mod tests {
    use crate::{read_input, find_number_of_easy_numbers_in_output};

    #[test]
    fn basic_test() {
        let (_, output_digits) = read_input("test_input.txt");
        let counter = find_number_of_easy_numbers_in_output(&output_digits);

        assert_eq!(counter, 26);
    }

    #[test]
    fn day8_part1() {
        let (_, output_digits) = read_input("input.txt");
        let counter = find_number_of_easy_numbers_in_output(&output_digits);

        assert_eq!(counter, 387);
    }
}
