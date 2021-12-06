use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let num_of_fish = simulate_lanternfish_growth(reader, 256);
    println!("{}", num_of_fish);
}

fn simulate_lanternfish_growth(reader: BufReader<File>, days: usize) -> usize {
    let input_string = reader.lines().next().unwrap().unwrap();
    let mut ages = vec![0; 9];
    for i in input_string.split(',').map(|f| f.parse::<usize>().unwrap()) {
        ages[i] += 1;
    }

    for _ in 0..days {
        let num_of_new_fish = ages[0];
        for i in 0..(ages.len() - 1) {
            ages[i] = ages[i + 1];
        }
        ages[6] += num_of_new_fish;
        ages[8] = num_of_new_fish;
    }

    ages.iter().sum()
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use crate::simulate_lanternfish_growth;

    #[test]
    fn basic_test1() {
        let file = File::open("basic_input.txt").unwrap();
        let reader = BufReader::new(file);
        let num_of_fish = simulate_lanternfish_growth(reader, 80);
        assert_eq!(num_of_fish, 5934);
    }

    #[test]
    fn basic_test2() {
        let file = File::open("basic_input.txt").unwrap();
        let reader = BufReader::new(file);
        let num_of_fish = simulate_lanternfish_growth(reader, 256);
        assert_eq!(num_of_fish, 26984457539);
    }

    #[test]
    fn day6_part1_test() {
        let file = File::open("input.txt").unwrap();
        let reader = BufReader::new(file);
        let num_of_fish = simulate_lanternfish_growth(reader, 80);
        assert_eq!(num_of_fish, 362346);
    }

    #[test]
    fn day6_part2_test() {
        let file = File::open("input.txt").unwrap();
        let reader = BufReader::new(file);
        let num_of_fish = simulate_lanternfish_growth(reader, 256);
        assert_eq!(num_of_fish, 1639643057051);
    }
}
