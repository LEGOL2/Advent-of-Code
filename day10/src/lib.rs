use core::panic;
use std::{fs::File, io::{BufRead, BufReader}};

static OPENING_CHARS: [char; 4] = ['(', '[', '{', '<'];

pub fn calculate_syntax_error_score(lines: &Vec<&str>) -> usize {
    let mut score = 0;

    for line in lines {
        is_valid(line, &mut score);
    }

    score
}

fn is_valid(line: &str, score: &mut usize) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for ch in line.chars() {
        if OPENING_CHARS.contains(&ch) {
            stack.push(ch);
        } else {
            if let Some(elem) = stack.last() {
                if is_matching(*elem, ch) {
                    stack.pop();
                } else {
                    println!("Expected {}, but found {} instead.", opposite(*elem), ch);
                    *score += char_to_score(ch);
                    return false;
                }
            }
        }
    }

    true
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

pub fn read_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse the line.")).collect()
}

#[cfg(test)]
mod tests {
    use crate::{is_valid, calculate_syntax_error_score, read_input};

    #[test]
    fn basic_tests() {
        let line = "([])";
        let mut score = 0;
        assert!(is_valid(line, &mut score));

        let line = "{()()()}";
        score = 0;
        assert!(is_valid(line, &mut score));

        let line = "<([{}])>";
        score = 0;
        assert!(is_valid(line, &mut score));

        let line = "[<>({}){}[([])<>]]";
        score = 0;
        assert!(is_valid(line, &mut score));

        let line = "(((((((((())))))))))";
        score = 0;
        assert!(is_valid(line, &mut score));
    }

    #[test]
    fn corrupted_chunks() {
        let line = "(]";
        let mut score = 0;
        assert!(!is_valid(line, &mut score));

        let line = "{()()()>";
        score = 0;
        assert!(!is_valid(line, &mut score));

        let line = "(((()))}";
        score = 0;
        assert!(!is_valid(line, &mut score));

        let line = "<([]){()}[{}])";
        score = 0;
        assert!(!is_valid(line, &mut score));
    }

    #[test]
    fn basic_sytax_error_test() {
        let lines = vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ];

        let score = calculate_syntax_error_score(&lines);
        assert_eq!(score, 26397);
    }

    #[test]
    fn day10_part1() {
        let input = read_input("input.txt");
        let lines = input.iter().map(|l| l.as_str()).collect();

        let score = calculate_syntax_error_score(&lines);
        assert_eq!(score, 318099);
    }
}
