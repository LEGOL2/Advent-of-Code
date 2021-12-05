use std::{io::{BufReader, BufRead, Lines}, fs::File};
mod structs;
use structs::{Board, BoardElement};

pub fn day4_part1(reader: BufReader<File>) -> usize {
    let mut lines = reader.lines();
    let input_string = lines.next().unwrap().unwrap();
    let numbers_to_call: Vec<usize> = input_string.split(',').map(|n| n.parse().unwrap()).collect();
    let board_elements = read_tables(&mut lines);
    let number_of_boards = board_elements.len() / 25;

    let mut boards: Vec<Board> = Vec::new();
    for i in 0..number_of_boards {
        let mut temporary_board_elemets = Vec::new();
        temporary_board_elemets.extend_from_slice(&board_elements[(25*i)..(25*i+25)]);
        boards.push(Board::new(temporary_board_elemets));
    }

    for called_number in numbers_to_call.iter() {
        for board in boards.iter_mut() {
            board.check_number(*called_number);
            if board.check_for_win() {
                let score = board.calculate_score();
                return score * called_number;
            }
        }
    }

    0 // No winning board
}

pub fn day4_part2(reader: BufReader<File>) -> usize {
    let mut lines = reader.lines();
    let input_string = lines.next().unwrap().unwrap();
    let numbers_to_call: Vec<usize> = input_string.split(',').map(|n| n.parse().unwrap()).collect();
    let board_elements = read_tables(&mut lines);
    let number_of_boards = board_elements.len() / 25;

    let mut boards: Vec<Board> = Vec::new();
    for i in 0..number_of_boards {
        let mut temporary_board_elemets = Vec::new();
        temporary_board_elemets.extend_from_slice(&board_elements[(25*i)..(25*i+25)]);
        boards.push(Board::new(temporary_board_elemets));
    }

    for called_number in numbers_to_call.iter() {
        let mut indicies_to_delete = Vec::new();
        for i in 0..boards.len() {
            let board = &mut boards[i];
            board.check_number(*called_number);
            if board.check_for_win() {
                board.has_won = true;
                indicies_to_delete.push(i);
            }
        }
        if boards.len() == 1 {
            if boards[0].has_won {
                return called_number * boards[0].calculate_score();
            }
            continue;
        }
        indicies_to_delete.sort();
        indicies_to_delete.reverse();
        for idx in indicies_to_delete {
            boards.remove(idx);
        }
    }

    0 // No winning board
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

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use crate::{day4_part1, day4_part2};

    #[test]
    fn basic_example_part1() {
        let file = File::open("example_input.txt").unwrap();
        let reader = BufReader::new(file);
        let score = day4_part1(reader);

        assert_eq!(score, 4512);
    }

    #[test]
    fn basic_example_part2() {
        let file = File::open("example_input.txt").unwrap();
        let reader = BufReader::new(file);
        let score = day4_part2(reader);

        assert_eq!(score, 1924);
    }

    #[test]
    fn day4_part1_test() {
        let file = File::open("input.txt").unwrap();
        let reader = BufReader::new(file);
        let score = day4_part1(reader);

        assert_eq!(score, 28082);
    }

    #[test]
    fn day4_part2_test() {
        let file = File::open("input.txt").unwrap();
        let reader = BufReader::new(file);
        let score = day4_part2(reader);

        assert_eq!(score, 8224);
    }
}
