use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn extended_polymerization(lines: &[String], iterations: i32) -> i64 {
    let (mut polymer, pair_insertion_rules, mut letters) = extract_template_and_rules(lines);

    for _ in 0..iterations {
        let mut tmp_polymer: HashMap<String, i64> = polymer.clone();
        for k in polymer.keys() {
            for _ in 0..*polymer.get(k).unwrap() {
                let element = pair_insertion_rules.get(k.as_str()).unwrap();

                if letters.contains_key(*element) {
                    let v = letters.get_mut(*element).unwrap();
                    *v += 1;
                } else {
                    letters.insert(element.to_string(), 1);
                }

                let mut tmp_key1 = String::from(k.get(0..1).unwrap());
                tmp_key1.push_str(&element);
                let mut tmp_key2 = String::from(*element);
                tmp_key2.push_str(k.get(1..).unwrap());

                if tmp_polymer.contains_key(&tmp_key1) {
                    let element = tmp_polymer.get_mut(&tmp_key1).unwrap();
                    *element += 1;
                } else {
                    tmp_polymer.insert(tmp_key1, 1);
                }

                if tmp_polymer.contains_key(&tmp_key2) {
                    let element = tmp_polymer.get_mut(&tmp_key2).unwrap();
                    *element += 1;
                } else {
                    tmp_polymer.insert(tmp_key2, 1);
                }

                let element = tmp_polymer.get_mut(k).unwrap();
                if *element > 0 {
                    *element -= 1;
                }
            }
        }

        polymer = tmp_polymer;
    }

    letters.values().max().unwrap() - letters.values().min().unwrap()
}

fn extract_template_and_rules(
    lines: &[String],
) -> (
    HashMap<String, i64>,
    HashMap<&str, &str>,
    HashMap<String, i64>,
) {
    let mut lines_iterator = lines.iter();
    let polymer_template = lines_iterator.next().unwrap();
    let mut letters = HashMap::new();

    let mut polymer = HashMap::new();
    let mut tmp_string = String::new();
    for ch in polymer_template.chars() {
        let k = String::from(ch);
        if letters.contains_key(&k) {
            let v = letters.get_mut(&k).unwrap();
            *v += 1;
        } else {
            letters.insert(k, 1);
        }
        match tmp_string.len() {
            0 => tmp_string.push(ch),
            1 => {
                tmp_string.push(ch);
                polymer.insert(tmp_string.clone(), 1);
                tmp_string.remove(0);
            }
            _ => panic!("Internal error."),
        }
    }

    lines_iterator.next().unwrap();
    let mut pair_insertion_rules = HashMap::new();
    for line in lines_iterator {
        let mut split = line.split(" -> ");
        pair_insertion_rules.insert(split.next().unwrap(), split.next().unwrap());
    }

    (polymer, pair_insertion_rules, letters)
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
    use super::*;

    #[test]
    fn basic_test_part1() {
        let lines = read_input("test_input.txt");

        assert_eq!(extended_polymerization(&lines, 10), 1588);
    }

    #[test]
    fn basic_test_part2() {
        let lines = read_input("test_input.txt");

        assert_eq!(extended_polymerization(&lines, 40), 2188189693529);
    }

    #[test]
    fn day14_part1() {
        let lines = read_input("input.txt");

        assert_eq!(extended_polymerization(&lines, 10), 2375);
    }
}
