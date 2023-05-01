use std::fs;

struct Position {
    vertical: i32,
    horizontal: i32,
    aim: i32,
}

enum Movement {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn add_1(pos: Position, rhs: Movement) -> Position {
    match rhs {
        Movement::Forward(x) => Position {
            vertical: pos.vertical,
            horizontal: pos.horizontal + x as i32,
            aim: pos.aim,
        },
        Movement::Up(x) => Position {
            vertical: pos.vertical - x as i32,
            horizontal: pos.horizontal,
            aim: pos.aim,
        },
        Movement::Down(x) => Position {
            vertical: pos.vertical + x as i32,
            horizontal: pos.horizontal,
            aim: pos.aim,
        },
    }
}

fn add_2(pos: Position, rhs: Movement) -> Position {
    match rhs {
        Movement::Forward(x) => Position {
            vertical: pos.vertical + pos.aim * x as i32,
            horizontal: pos.horizontal + x as i32,
            aim: pos.aim,
        },
        Movement::Up(x) => Position {
            vertical: pos.vertical,
            horizontal: pos.horizontal,
            aim: pos.aim - x as i32,
        },
        Movement::Down(x) => Position {
            vertical: pos.vertical,
            horizontal: pos.horizontal,
            aim: pos.aim + x as i32,
        },
    }
}

fn get_movement(line: &str) -> Movement {
    let v: Vec<&str> = line.split(" ").collect();
    if let [direction_str, distance_str] = &v[..] {
        let distance = distance_str.parse::<u32>().unwrap();
        match *direction_str {
            "forward" => Movement::Forward(distance),
            "down" => Movement::Down(distance),
            "up" => Movement::Up(distance),
            _ => panic!("Line '{}' did not conform to the expected format", line),
        }
    } else {
        panic!("Line '{}' did not conform to the expected format", line)
    }
}

fn result_to_string(p: Position) -> String {
    (p.vertical * p.horizontal).to_string()
}

fn general_solution<F: Fn(Position, Movement) -> Position>(adder: F) -> String {
    // get starting position
    let origin = Position {
        vertical: 0,
        horizontal: 0,
        aim: 0,
    };
    // load file
    let input: String = fs::read_to_string("inputs/day2.txt").expect("Could not read file");
    // go through lines one by one
    let result: Position = input
        .lines()
        .map(|line| get_movement(line))
        .fold(origin, |acc, m| adder(acc, m));
    // return resulting string
    result_to_string(result)
}

pub fn ex1() -> String {
    general_solution(add_1)
}

pub fn ex2() -> String {
    general_solution(add_2)
}

#[cfg(test)]
mod tests {
    use super::{ex1, ex2};
    #[test]
    fn test_both_exercises() {
        assert_eq!(ex1(), "1990000");
        assert_eq!(ex2(), "1975421260");
    }
}
