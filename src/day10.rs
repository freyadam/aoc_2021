use itertools::Itertools;
use std::collections::VecDeque;
use std::fs;

fn corrupted_char_value(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unexpected symbol"),
    }
}

fn autocomplete_char_value(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("Unexpected symbol"),
    }
}

fn expected_symbol(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("Unexpected symbol"),
    }
}

fn corrupted_line_value(line: &str) -> Option<u32> {
    let mut deque: VecDeque<char> = VecDeque::new();

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => deque.push_front(c),
            _ => {
                let d: Option<char> = deque.pop_front();
                if d.is_some() {
                    let d_char: char = d.unwrap();
                    if d_char != expected_symbol(c) {
                        return Some(corrupted_char_value(c));
                    }
                } else {
                    return None;
                }
            }
        }
    }
    None
}

fn autocomplete_value(line: &str) -> u64 {
    let mut deque: VecDeque<char> = VecDeque::new();

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => {
                deque.push_front(c);
            }
            _ => {
                deque.pop_front();
            }
        }
    }

    let mut value: u64 = 0;

    while let Some(c) = deque.pop_front() {
        value = 5 * value + autocomplete_char_value(c);
    }

    value
}

pub fn ex1() -> String {
    let input_str: String = fs::read_to_string("inputs/day10.txt").expect("Could not read file");

    input_str
        .lines()
        .map(|ln| corrupted_line_value(ln))
        .filter(|opt| opt.is_some())
        .map(|opt| opt.unwrap())
        .fold(0, |acc, k| acc + k)
        .to_string()
}

pub fn ex2() -> String {
    let input_str: String = fs::read_to_string("inputs/day10.txt").expect("Could not read file");

    let values: Vec<u64> = input_str
        .lines()
        .filter(|ln| corrupted_line_value(ln).is_none())
        .map(|ln| autocomplete_value(ln))
        .sorted()
        .collect();

    values[values.len() / 2].to_string()
}

#[cfg(test)]
mod tests {
    use super::{ex1, ex2};
    #[test]
    fn test_both_exercises() {
        assert_eq!(ex1(), "339477");
        assert_eq!(ex2(), "3049320156");
    }
}
