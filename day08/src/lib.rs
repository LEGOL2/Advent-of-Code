use std::{fs::File, io::{BufReader, BufRead}};

mod display;
use display::*;

pub fn find_number_of_easy_numbers_in_output_digits(output_numbers: &Vec<Vec<String>>) -> usize {
    let mut counter = 0;

    for number in output_numbers.iter() {
        for digit in number.iter() {
            match digit.len() {
                2 | 3 | 4 | 7 => counter += 1,
                _ => (),
            }
        }
    }

    counter
}

pub fn foo(input_values: &Vec<Vec<String>>, output_values: &Vec<Vec<String>>) -> u32 {
    let (input_numbers, output_numbers) = convert_strings_into_segments(input_values, output_values);
    let mut scores: Vec<u32> = Vec::new();

    // Iterate over each line of input data
    for (input_line, output_line) in input_numbers.iter().zip(output_numbers.iter()) {
        // Find 1 and 4
        let one = input_line.iter().find(|v| v.len() == 2).unwrap();
        let four = input_line.iter().find(|v| v.len() == 4).unwrap();
        
        // Find 4 - 1
        let fourdiff: Vec<&Segment> = four.iter().filter(|s| !one.contains(s)).collect();

        // Find 2, 3, 5
        let three = input_line.iter().find(|v| {
            v.len() == 5 && one.iter().all(|s| v.contains(s))
        }).unwrap();
        let five = input_line.iter().find(|v| {
            v.len() == 5 && fourdiff.iter().all(|s| v.contains(s))
        }).unwrap();
        let two = input_line.iter().find(|v| {
            v.len() == 5 && five.iter().any(|s| !v.contains(s)) && three.iter().any(|s| !v.contains(s))
        }).unwrap();

        // Find 6 and 9
        let six = input_line.iter().find(|v| {
            v.len() == 6 && fourdiff.iter().all(|s| v.contains(s)) && !four.iter().all(|s| v.contains(s))
        }).unwrap();
        let nine = input_line.iter().find(|v| {
            v.len() == 6 && four.iter().all(|s| v.contains(s))
        }).unwrap();


        // Convert output to numbers
        let mut score: u32 = 0;
        for i in 0..4 {
            match output_line[i].len() {
                2 => score += 10_u32.pow(3-i as u32),
                3 => score += 7*10_u32.pow(3-i as u32),
                4 => score += 4*10_u32.pow(3-i as u32),
                5 => {
                    let output_three = three.iter().all(|s| output_line[i].contains(s));
                    if output_three {
                        score += 3*10_u32.pow(3-i as u32);
                        continue;
                    }

                    let output_five = five.iter().all(|s| output_line[i].contains(s));
                    if output_five {
                        score += 5*10_u32.pow(3-i as u32);
                        continue;
                    }

                    let output_two = two.iter().all(|s| output_line[i].contains(s));
                    if output_two {
                        score += 2*10_u32.pow(3-i as u32);
                        continue;
                    }
                },
                6 => {
                    let output_six = six.iter().all(|s| output_line[i].contains(s));
                    if output_six {
                        score += 6*10_u32.pow(3-i as u32);
                        continue;
                    }

                    let output_nine = nine.iter().all(|s| output_line[i].contains(s));
                    if output_nine {
                        score += 9*10_u32.pow(3-i as u32);
                        continue;
                    }
                },
                7 => score += 8*10_u32.pow(3-i as u32),
                _ => panic!(),
            }
        }

        scores.push(score);
        println!("{}", score);
    }

    scores.iter().sum()
}

fn convert_strings_into_segments(input_values: &Vec<Vec<String>>, output_values: &Vec<Vec<String>>) -> (Vec<Vec<Vec<Segment>>>, Vec<Vec<Vec<Segment>>>) {
    let input_numbers: Vec<Vec<Vec<Segment>>> = input_values.iter().map(|v| {
        v.iter().map(|s| {
            s.chars().map(|ch| {
                Segment::from_char(ch)
            }).collect::<Vec<Segment>>()
        }).collect::<Vec<Vec<Segment>>>()
    }).collect();
    let output_numbers: Vec<Vec<Vec<Segment>>> = output_values.iter().map(|v| {
        v.iter().map(|s| {
            s.chars().map(|ch| {
                Segment::from_char(ch)
            }).collect::<Vec<Segment>>()
        }).collect::<Vec<Vec<Segment>>>()
    }).collect();

    (input_numbers, output_numbers)
}

pub fn read_input(filename: &str) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines()
        .map(|l| l.expect("Could not parse the line."))
        .collect();
    
    let mut signal_patterns: Vec<Vec<String>> = Vec::new();
    let mut output_digits: Vec<Vec<String>> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split('|').collect();
        signal_patterns.push(parts[0].strip_suffix(' ').unwrap().split(' ').map(|s| s.to_owned()).collect::<Vec<String>>());
        output_digits.push(parts[1].strip_prefix(' ').unwrap().split(' ').map(|s| s.to_owned()).collect::<Vec<String>>());
    }

    (signal_patterns, output_digits)
}

#[cfg(test)]
mod tests {
    use crate::{read_input, find_number_of_easy_numbers_in_output_digits, foo};

    #[test]
    fn basic_test() {
        let (_, output_digits) = read_input("test_input.txt");
        let counter = find_number_of_easy_numbers_in_output_digits(&output_digits);

        assert_eq!(counter, 26);
    }

    #[test]
    fn example() {
        let (input_numbers, output_numbers) = read_input("example.txt");
        let score = foo(&input_numbers, &output_numbers);

        assert_eq!(score, 8394);
    }

    #[test]
    fn basic_test_part2() {
        let (input_numbers, output_numbers) = read_input("test_input.txt");
        let score = foo(&input_numbers, &output_numbers);

        assert_eq!(score, 61229)
    }

    #[test]
    fn day8_part1() {
        let (_, output_digits) = read_input("input.txt");
        let counter = find_number_of_easy_numbers_in_output_digits(&output_digits);

        assert_eq!(counter, 387);
    }

    #[test]
    fn day8_part2() {
        let (input_numbers, output_numbers) = read_input("input.txt");
        let score = foo(&input_numbers, &output_numbers);

        assert_eq!(score, 986034);
    }
}
