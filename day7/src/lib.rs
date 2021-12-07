use std::fs::read_to_string;

pub fn day7_part1(file: &str) -> usize {
    let positions: Vec<usize> = read_to_string(file)
        .unwrap()
        .trim()
        .split(',')
        .map(|f| f.parse().unwrap())
        .collect();
    let max_position = *positions.iter().max().unwrap();
    let mut costs = vec![usize::MAX; max_position];

    for pos in 0..max_position {
        let mut cost = 0;
        for elem in positions.iter() {
            cost += abs_diff(*elem, pos);
        }

        costs[pos] = cost;
    }

    *costs.iter().min().unwrap()
}

pub fn day7_part2(file: &str) -> usize {
    let positions: Vec<usize> = read_to_string(file)
        .unwrap()
        .trim()
        .split(',')
        .map(|f| f.parse().unwrap())
        .collect();
    let max_position = *positions.iter().max().unwrap();
    let mut costs = vec![usize::MAX; max_position];

    for pos in 0..max_position {
        let mut cost = 0;
        for elem in positions.iter() {
            cost += sum_of_numbers(*elem, pos);
        }

        costs[pos] = cost;
    }

    *costs.iter().min().unwrap()
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn sum_of_numbers(a: usize, b: usize) -> usize {
    let n = abs_diff(a, b) as f32;
    (n / 2_f32 * (n + 1_f32)) as usize
}

#[cfg(test)]
mod tests {
    use crate::{day7_part1, day7_part2, sum_of_numbers};

    #[test]
    fn basic_example_part1() {
        let fuel = day7_part1("example_input.txt");
        assert_eq!(fuel, 37);
    }

    #[test]
    fn basic_example_part2() {
        let fuel = day7_part2("example_input.txt");
        assert_eq!(fuel, 168);
    }

    #[test]
    fn sum_of_numbers_tests() {
        let result = sum_of_numbers(16, 5);
        assert_eq!(result, 66);

        let result = sum_of_numbers(1, 5);
        assert_eq!(result, 10);

        let result = sum_of_numbers(2, 5);
        assert_eq!(result, 6);

        let result = sum_of_numbers(7, 5);
        assert_eq!(result, 3);
    }

    #[test]
    fn day7_part1_test() {
        let fuel = day7_part1("input.txt");
        assert_eq!(fuel, 335271);
    }

    #[test]
    fn day7_part2_test() {
        let fuel = day7_part2("input.txt");
        assert_eq!(fuel, 95851339);
    }
}
