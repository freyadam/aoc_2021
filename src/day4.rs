use std::fs;
use std::iter::Iterator;
use std::str::Lines;

#[derive(Debug)]
struct Board {
    pub numbers: [[u8; 5]; 5],
    pub hits: [[bool; 5]; 5],
}

impl Board {
    pub fn new() -> Board {
        Board {
            numbers: [[0u8; 5]; 5],
            hits: [[false; 5]; 5],
        }
    }

    pub fn winning(&self) -> bool {
        let mut result = false;

        for i in 0..5 {
            result |= self.hits[i][0]
                & self.hits[i][1]
                & self.hits[i][2]
                & self.hits[i][3]
                & self.hits[i][4];
        }

        for i in 0..5 {
            result |= self.hits[0][i]
                & self.hits[1][i]
                & self.hits[2][i]
                & self.hits[3][i]
                & self.hits[4][i];
        }

        result
    }
}

fn load_numbers(line: Option<&str>) -> Vec<u8> {
    line.expect("")
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect()
}

fn load_boards(mut lines: Lines) -> Vec<Board> {
    let mut boards = Vec::new();

    while let Some(_) = lines.next() {
        let mut board = Board::new();

        for i in 0..5 {
            board.numbers[i] = lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();
        }

        boards.push(board);
    }

    boards
}

fn score(board: &Board, number: u8) -> u32 {
    let mut sum: u32 = 0;

    for i in 0..5 {
        for j in 0..5 {
            if !board.hits[i][j] {
                sum += board.numbers[i][j] as u32;
            }
        }
    }
    sum * (number as u32)
}

fn general_solution(last_board: bool) -> String {
    let input_str = fs::read_to_string("inputs/day4.txt").expect("Could not read file");
    let mut lines = input_str.lines();
    let numbers = load_numbers(lines.next());
    let mut boards: Vec<Board> = load_boards(lines);

    for number in numbers {
        for idx in 0..boards.len() {
            for i in 0..5 {
                for j in 0..5 {
                    if boards[idx].numbers[i][j] == number {
                        boards[idx].hits[i][j] = true;
                    }
                }
            }
        }

        for idx in (0..boards.len()).rev() {
            if boards[idx].winning() {
                if !last_board {
                    return score(&boards[idx], number).to_string();
                } else {
                    if boards.len() == 1 {
                        return score(&boards[0], number).to_string();
                    } else {
                        boards.remove(idx);
                    }
                }
            }
        }
    }

    panic!("No winners, undefined score")
}

pub fn ex1() -> String {
    general_solution(false)
}

pub fn ex2() -> String {
    general_solution(true)
}
