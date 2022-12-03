use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

static OPENING_CHARS: [char; 4] = ['(', '[', '{', '<'];

pub fn calculate_scores(lines: &Vec<String>) -> (usize, usize) {
    let mut error_score = 0;
    let mut autocomplete_scores: Vec<usize> = Vec::new();

    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        if let Some(ch) = is_valid(line, &mut stack) {
            // Corrupted line, add error score.
            error_score += char_to_score(ch);
        } else {
            // Incomplete line, add autocompletion score.
            autocomplete_scores.push(calculate_autocomplete_score(&mut stack));
        }
    }

    autocomplete_scores.sort();
    let idx = autocomplete_scores.len() / 2;

    (error_score, autocomplete_scores[idx])
}

pub fn calculate_autocomplete_score(stack: &mut Vec<char>) -> usize {
    let mut score = 0;
    for i in (0..stack.len()).rev() {
        score *= 5;
        score += autocomplete_char_to_score(opposite(stack[i]));
    }

    score
}

fn is_valid(line: &str, stack: &mut Vec<char>) -> Option<char> {
    for ch in line.chars() {
        if OPENING_CHARS.contains(&ch) {
            stack.push(ch);
        } else {
            if let Some(elem) = stack.last() {
                if is_matching(*elem, ch) {
                    stack.pop();
                } else {
                    println!("Expected {}, but found {} instead.", opposite(*elem), ch);
                    return Some(ch);
                }
            }
        }
    }

    None
}

fn is_matching(opening: char, closing: char) -> bool {
    match (opening, closing) {
        ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => true,
        _ => false,
    }
}

fn opposite(ch: char) -> char {
    match ch {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Illegal character!"),
    }
}

fn char_to_score(ch: char) -> usize {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Illegal character!"),
    }
}

fn autocomplete_char_to_score(ch: char) -> usize {
    match ch {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Illegal character!"),
    }
}

pub fn read_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse the line."))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{calculate_scores, read_input};

    #[test]
    fn basic_test() {
        let lines = read_input("test_input.txt");

        let (error_score, autocomplete_score) = calculate_scores(&lines);
        assert_eq!(error_score, 26397);
        assert_eq!(autocomplete_score, 288957);
    }

    #[test]
    fn day10() {
        let lines = read_input("input.txt");

        let (error_score, autocomplete_score) = calculate_scores(&lines);
        assert_eq!(error_score, 318099);
        assert_eq!(autocomplete_score, 2389738699);
    }
}
