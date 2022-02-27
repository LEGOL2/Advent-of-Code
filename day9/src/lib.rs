use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn smoke_basin(input: Vec<u32>, width: usize, height: usize) -> usize {
    let mut low_points: Vec<u32> = Vec::new();

    for (idx, elem) in input.iter().enumerate() {
        if idx % width > 0 {
            if *elem >= input[idx - 1] {
                continue;
            }
        }

        if idx % width < width - 1 {
            if *elem >= input[idx + 1] {
                continue;
            }
        }

        if idx / width > 0 {
            if *elem >= input[idx - width] {
                continue;
            }
        }

        if idx / width < height - 1 {
            if *elem >= input[idx + width] {
                continue;
            }
        }

        low_points.push(*elem);
    }

    low_points.iter().fold(0, |acc, p| acc + p) as usize + low_points.len()
}

pub fn read_input(filename: &str) -> Vec<u32> {
    let file = File::open(filename).unwrap();
    let mut input: Vec<u32> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let mut data = line
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        input.append(&mut data);
    }

    input
}

#[cfg(test)]
mod tests {
    use crate::{read_input, smoke_basin};

    #[test]
    fn example_part1() {
        let input = read_input("test_input.txt");
        let risk_level = smoke_basin(input, 10, 5);
        assert_eq!(risk_level, 15);
    }

    #[test]
    fn part1() {
        let input = read_input("input.txt");
        let risk_level = smoke_basin(input, 100, 100);
        assert_eq!(risk_level, 575);
    }
}
