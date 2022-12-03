use std::fs::File;
use std::io::{self, BufRead};

fn read_input(path: &str) -> Vec<u32> {
    let file = File::open(path).unwrap();
    let mut sums = Vec::new();
    let mut temp_sum = 0;

    for line in io::BufReader::new(file).lines() {
        let text = line.unwrap();
        if text != "" {
            let number: u32 = text.parse().unwrap();
            temp_sum += number;
        } else {
            sums.push(temp_sum);
            temp_sum = 0;
        }
    }
    sums.push(temp_sum);

    sums
}

pub fn find_max_calories(path: &str) -> u32 {
    let sums = read_input(path);

    *sums.iter().max().unwrap()
}

pub fn find_top_3_calories(path: &str) -> u32 {
    let mut sums = read_input(path);

    sums.sort();
    sums.reverse();

    sums[0..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_example() {
        let calories = find_max_calories("test_input.txt");
        let top_3_calories = find_top_3_calories("test_input.txt");
        assert_eq!(calories, 24000);
        assert_eq!(top_3_calories, 45000);
    }

    #[test]
    fn day01_part01() {
        let calories = find_max_calories("input.txt");
        assert_eq!(calories, 66306);
    }

    #[test]
    fn day01_part02() {
        let top_3_calories = find_top_3_calories("input.txt");
        assert_eq!(top_3_calories, 195292);
    }
}
