use std::{io::{BufReader, BufRead, Lines}, fs::File};

struct BoardElement {
    number: usize,
    was_called: bool
}

impl BoardElement {
    fn new(number: usize) -> Self {
        BoardElement {
            number,
            was_called: false
        }
    }
}

pub fn day4_part1(reader: BufReader<File>) -> usize {
    let mut lines = reader.lines();
    let input_string = lines.next().unwrap().unwrap();
    let numbers_to_call: Vec<usize> = input_string.split(',').map(|n| n.parse().unwrap()).collect();
    let mut board_elements = read_tables(&mut lines);
    let number_of_boards = board_elements.len() / 25;
    
    for called_number in numbers_to_call.iter() {
        for board in 0..number_of_boards {
            for i in 0..25 {
                let board_element = &mut board_elements[25*board + i];
                if board_element.number == *called_number {
                    board_element.was_called = true;
                }
            }
        }
    }

    check_for_wins(&board_elements, number_of_boards);

    0
}

fn read_tables(lines: &mut Lines<BufReader<File>>) -> Vec<BoardElement> {
    let mut numbers: Vec<BoardElement> = Vec::new();
    loop {
        let line = lines.next();
        match line {
            Some(line) => {
                let element = line.unwrap();
                for item in element.split(' ') {
                    if item.is_empty() {
                        continue;
                    }
                    let number: usize = item.parse().unwrap();
                    numbers.push(BoardElement::new(number));
                }
            },
            None => return numbers,
        }
    }
}

fn check_for_wins(board_elements: &Vec<BoardElement>, number_of_boards: usize) {
    for i in 0..5 {
        
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
