use itertools::Itertools;
use std::collections::VecDeque;
use std::fs;

type Row = Vec<u8>;
type Board = Vec<Row>;

#[derive(Debug)]
struct Position {
    y: usize,
    x: usize,
}

struct BoardIterator {
    x: usize,
    y: usize,
    board_width: usize,
    board_height: usize,
}

impl BoardIterator {
    fn new(board: &Board) -> BoardIterator {
        BoardIterator {
            x: 0,
            y: 0,
            board_height: board.len(),
            board_width: if board.len() == 0 { 0 } else { board[0].len() },
        }
    }
}

impl Iterator for BoardIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y == self.board_height {
            None
        } else if self.x == self.board_width - 1 {
            self.x = 0;
            self.y += 1;
            Some(Position {
                x: self.board_width - 1,
                y: self.y - 1,
            })
        } else {
            self.x += 1;
            Some(Position {
                x: self.x - 1,
                y: self.y,
            })
        }
    }
}

fn load_row(line: &str) -> Row {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn load_board(s: String) -> Board {
    let mut board: Board = Board::new();

    for line in s.lines() {
        board.push(load_row(line))
    }

    board
}

fn is_low_point(board: &Board, pos: &Position) -> bool {
    let mut is_low: bool = true;

    if 0 < pos.x {
        is_low &= board[pos.y][pos.x - 1] > board[pos.y][pos.x];
    }
    if pos.x < board[0].len() - 1 {
        is_low &= board[pos.y][pos.x] < board[pos.y][pos.x + 1];
    }
    if 0 < pos.y {
        is_low &= board[pos.y - 1][pos.x] > board[pos.y][pos.x];
    }
    if pos.y < board.len() - 1 {
        is_low &= board[pos.y][pos.x] < board[pos.y + 1][pos.x];
    }

    is_low
}

fn risk_level(board: &Board, pos: &Position) -> u16 {
    1 + board[pos.y][pos.x] as u16
}

fn basin_size(board: &Board, pos: &Position) -> u32 {
    let mut queue: VecDeque<Position> = VecDeque::from([Position { x: pos.x, y: pos.y }]);
    let mut opened: Vec<Vec<bool>> = Vec::new();
    for board_row in board {
        let row: Vec<bool> = board_row.clone().iter().map(|_| false).collect();
        opened.push(row)
    }

    // breadth-first search of the area around specified low point
    while let Some(curr) = queue.pop_front() {
        opened[curr.y][curr.x] = true;

        if
        // position is on the board
        0 < curr.x
        // position was not yet opened beforehand
        && !opened[curr.y][curr.x - 1]
        // position is not lower than its predecessor
        && board[curr.y][curr.x - 1] >= board[curr.y][curr.x]
        // position is not maximum
        && board[curr.y][curr.x - 1] != 9
        {
            queue.push_back(Position {
                y: curr.y,
                x: curr.x - 1,
            })
        }
        if curr.x < board[0].len() - 1
            && !opened[curr.y][curr.x + 1]
            && board[curr.y][curr.x + 1] >= board[curr.y][curr.x]
            && board[curr.y][curr.x + 1] != 9
        {
            queue.push_back(Position {
                y: curr.y,
                x: curr.x + 1,
            })
        }
        if 0 < curr.y
            && !opened[curr.y - 1][curr.x]
            && board[curr.y - 1][curr.x] >= board[curr.y][curr.x]
            && board[curr.y - 1][curr.x] != 9
        {
            queue.push_back(Position {
                y: curr.y - 1,
                x: curr.x,
            })
        }
        if curr.y < board.len() - 1
            && !opened[curr.y + 1][curr.x]
            && board[curr.y + 1][curr.x] >= board[curr.y][curr.x]
            && board[curr.y + 1][curr.x] != 9
        {
            queue.push_back(Position {
                y: curr.y + 1,
                x: curr.x,
            })
        }
    }

    // calculate and return volume
    let mut volume: u32 = 0;
    for row in opened {
        for is_opened in row {
            volume += if is_opened { 1 } else { 0 };
        }
    }
    volume
}

pub fn ex1() -> String {
    let input_str: String = fs::read_to_string("inputs/day9.txt").expect("Could not read file");
    let board: Board = load_board(input_str);

    BoardIterator::new(&board)
        .filter(|pos| is_low_point(&board, &pos))
        .map(|pos| risk_level(&board, &pos))
        .sum::<u16>()
        .to_string()
}

pub fn ex2() -> String {
    let input_str: String = fs::read_to_string("inputs/day9.txt").expect("Could not read file");
    let board: Board = load_board(input_str);

    BoardIterator::new(&board)
        .filter(|pos| is_low_point(&board, &pos))
        .map(|pos| basin_size(&board, &pos))
        .sorted_by(|a, b| a.cmp(&b).reverse())
        .take(3)
        .reduce(|acc, k| acc * k)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{ex1, ex2};
    #[test]
    fn test_both_exercises() {
        assert_eq!(ex1(), "468");
        assert_eq!(ex2(), "1280496");
    }
}
