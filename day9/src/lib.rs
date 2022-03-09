use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn smoke_basin(input: Vec<u32>, width: usize, height: usize) -> usize {
    let mut low_points: Vec<(u32, u32)> = Vec::new();
    find_low_points(&input, width, height, &mut low_points);

    low_points.iter().fold(0, |acc, p| acc + p.1) as usize + low_points.len()
}

pub fn find_three_largest_basins(
    mut input: Vec<u32>,
    mut width: usize,
    mut height: usize,
) -> usize {
    let mut low_points: Vec<(u32, u32)> = Vec::new();
    surround_with_9(&mut input, &mut width, &mut height);
    find_low_points(&input, width, height, &mut low_points);

    fill_basins(&mut input, width, &mut low_points)
}

fn find_low_points(
    input: &Vec<u32>,
    width: usize,
    height: usize,
    low_points: &mut Vec<(u32, u32)>,
) {
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

        low_points.push((idx as u32, *elem));
    }
}

fn surround_with_9(input: &mut Vec<u32>, width: &mut usize, height: &mut usize) {
    *width += 2;
    *height += 2;
    for i in 0..*width {
        input.insert(i, 9);
        input.push(9);
    }
    for j in 1..*height - 1 {
        input.insert(*width * j, 9);
        input.insert(*width * j + *width - 1, 9);
    }
}

fn fill_basins(input: &mut Vec<u32>, width: usize, low_points: &mut Vec<(u32, u32)>) -> usize {
    let mut sizes: Vec<usize> = Vec::new();
    for (idx, _) in low_points.iter() {
        let mut counter = 0;
        flood_fill(input, *idx, width, 9, &mut counter);
        sizes.push(counter);
    }

    sizes.sort();
    sizes.reverse();
    sizes[0] * sizes[1] * sizes[2]
}

fn flood_fill(input: &mut Vec<u32>, idx: u32, width: usize, new_value: u32, counter: &mut usize) {
    if idx as usize > input.len() {
        return;
    }

    if input[idx as usize] == new_value {
        return;
    }

    input[idx as usize] = new_value;
    *counter += 1;

    flood_fill(input, idx + 1, width, new_value, counter);
    flood_fill(input, idx + width as u32, width, new_value, counter);
    flood_fill(input, idx - 1, width, new_value, counter);
    flood_fill(input, idx - width as u32, width, new_value, counter);
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
    use crate::{find_three_largest_basins, read_input, smoke_basin};

    #[test]
    fn example_part1() {
        let input = read_input("test_input.txt");
        let risk_level = smoke_basin(input, 10, 5);
        assert_eq!(risk_level, 15);
    }

    #[test]
    fn example_part2() {
        let input = read_input("test_input.txt");
        let size = find_three_largest_basins(input, 10, 5);
        assert_eq!(size, 1134);
    }

    #[test]
    fn part1() {
        let input = read_input("input.txt");
        let risk_level = smoke_basin(input, 100, 100);
        assert_eq!(risk_level, 575);
    }

    #[test]
    fn part2() {
        let input = read_input("input.txt");
        let size = find_three_largest_basins(input, 100, 100);
        assert_eq!(size, 1019700);
    }
}
