use std::{io::{BufReader, BufRead, Lines}, fs::File};

pub fn day4_part1(reader: BufReader<File>) -> usize {
    let mut lines = reader.lines();
    let input_string = lines.next().unwrap().unwrap();
    let numbers: Vec<usize> = input_string.split(',').map(|n| n.parse().unwrap()).collect();
    
    read_tables(&mut lines);

    0
}

fn read_tables(lines: &mut Lines<BufReader<File>>) {
    lines.next();
    for i in 0..5 {
        let line = lines.next().unwrap().unwrap();
        let numbers: Vec<usize> = line.split(' ').map(|n| n.parse().unwrap()).collect();
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use crate::day4_part1;

    #[test]
    fn basic_example_part1() {
        let file = File::open("example_input.txt").unwrap();
        let reader = BufReader::new(file);
        let score = day4_part1(reader);

        assert_eq!(score, 4512);
    }
}
