use std::fs::File;
use std::io::{self, BufRead};

mod data;
use data::Data;

pub fn part1(path: &str) -> usize {
    let input = read_input(path);
    const SIZE: usize = 1000;
    let mut field = vec![0; SIZE * SIZE];
    for elem in input.iter() {
        let min_x = elem.min_x();
        let min_y = elem.min_y();

        if elem.x1 == elem.x2 {
            for i in 0..=abs(elem.y1, elem.y2) {
                field[(min_y + i) as usize * SIZE + min_x as usize] += 1;
            }
        } else if elem.y1 == elem.y2 {
            let y = min_y * SIZE as isize;
            for i in 0..=abs(elem.x1, elem.x2) {
                field[(y + min_x + i) as usize] += 1;
            }
        }
    }

    let mut counter = 0;
    for item in field.iter() {
        if *item > 1 {
            counter += 1;
        }
    }

    counter
}

pub fn hydrothermal_venture(path: &str, is_part_2: bool) -> usize {
    let input = read_input(path);
    const SIZE: usize = 1000;
    let mut field = vec![0; SIZE * SIZE];

    for elem in input.iter() {
        let min_x = elem.min_x();
        let min_y = elem.min_y();

        let a = elem.slope();
        // y = ax + b -> b = y - ax
        let b = elem.y1 - a * elem.x1;

        for i in 0..=elem.distance() {
            let (x, y);
            if elem.x1 == elem.x2 {
                // Vertical line
                x = min_x;
                y = min_y + i;
            } else if elem.y1 == elem.y2 {
                // Horizontal line
                x = min_x + i;
                y = min_y;
            } else {
                if is_part_2 {
                    x = min_x + i;
                    y = a * x + b;
                } else {
                    continue;
                }
            }

            let idx = y as usize * SIZE + x as usize;
            field[idx] += 1;
        }
    }

    let mut counter = 0;
    for item in field.iter() {
        if *item > 1 {
            counter += 1;
        }
    }

    counter
}

fn read_input(path: &str) -> Vec<Data> {
    let file = File::open(path).unwrap();
    let mut data = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let text = line.unwrap();

        let mut it = text.split(" -> ");
        let mut p1 = it.next().unwrap().split(",");
        let x1 = p1.next().unwrap().parse().unwrap();
        let y1 = p1.next().unwrap().parse().unwrap();

        let mut p2 = it.next().unwrap().split(",");
        let x2 = p2.next().unwrap().parse().unwrap();
        let y2 = p2.next().unwrap().parse().unwrap();

        data.push(Data::new(x1, y1, x2, y2));
    }

    data
}

fn abs<T: std::cmp::PartialOrd + std::ops::Sub<Output = T>>(a: T, b: T) -> T {
    if a >= b {
        a - b
    } else {
        b - a
    }
}

#[cfg(test)]
mod tests {
    use crate::hydrothermal_venture;

    #[test]
    fn basic_example_part1() {
        let counter = hydrothermal_venture("test_input.txt", false);
        assert_eq!(counter, 5);
    }

    #[test]
    fn basic_example_part2() {
        let counter = hydrothermal_venture("test_input.txt", true);
        assert_eq!(counter, 12);
    }

    #[test]
    fn part1_test() {
        let counter = hydrothermal_venture("input.txt", false);
        assert_eq!(counter, 5632);
    }

    #[test]
    fn part2_test() {
        let counter = hydrothermal_venture("input.txt", true);
        assert_eq!(counter, 22213)
    }
}
