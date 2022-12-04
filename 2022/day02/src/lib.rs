use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy)]
enum Move {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

pub fn calculate_score_from_strategy_part1(path: &str) -> usize {
    let moves = read_input(path);

    moves
        .iter()
        .map(|(opponent_move, my_move)| -> usize { get_score1(opponent_move, my_move) })
        .sum()
}

pub fn calculate_score_from_strategy_part2(path: &str) -> usize {
    let moves = read_input(path);

    moves
        .iter()
        .map(|(opponent_move, my_move)| -> usize { get_score2(opponent_move, my_move) })
        .sum()
}

fn get_score1(opponent: &Move, player: &Move) -> usize {
    const SCORE_TABLE: [usize; 9] = [4, 8, 3, 1, 5, 9, 7, 2, 6];
    let idx = *opponent as usize * 3 + *player as usize;

    SCORE_TABLE[idx]
}

fn get_score2(opponent: &Move, player: &Move) -> usize {
    const SCORE_TABLE: [usize; 9] = [3, 4, 8, 1, 5, 9, 2, 6, 7];
    let idx = *opponent as usize * 3 + *player as usize;

    SCORE_TABLE[idx]
}

fn read_input(path: &str) -> Vec<(Move, Move)> {
    let file = File::open(path).unwrap();

    BufReader::new(file)
        .lines()
        .map(|line| -> (Move, Move) {
            let l = line.unwrap();
            let inputs: Vec<&str> = l.split(' ').collect();

            let opponent_move = match inputs[0] {
                "A" => Move::Rock,
                "B" => Move::Paper,
                "C" => Move::Scissors,
                _ => panic!(),
            };

            let my_move = match inputs[1] {
                "X" => Move::Rock,
                "Y" => Move::Paper,
                "Z" => Move::Scissors,
                _ => panic!(),
            };

            (opponent_move, my_move)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_part1() {
        let score = calculate_score_from_strategy_part1("test_input.txt");
        assert_eq!(score, 15)
    }

    #[test]
    fn basic_test_part2() {
        let score = calculate_score_from_strategy_part2("test_input.txt");
        assert_eq!(score, 12)
    }

    #[test]
    fn day02_part1() {
        let score = calculate_score_from_strategy_part1("input.txt");
        assert_eq!(score, 9759)
    }

    #[test]
    fn day02_part2() {
        let score = calculate_score_from_strategy_part2("input.txt");
        assert_eq!(score, 12429)
    }
}
