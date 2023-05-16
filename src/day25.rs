use std::collections::BTreeSet;
use std::fs;

type Position = (u32, u32);

#[derive(Debug, Eq, PartialEq)]
struct State {
    board_width: u32,
    board_height: u32,
    east_herd: BTreeSet<Position>,
    south_herd: BTreeSet<Position>,
}

impl State {
    fn from(s: String) -> State {
        let mut y: u32 = 0;
        let mut x: u32 = 0;
        let mut east_herd: BTreeSet<Position> = BTreeSet::new();
        let mut south_herd: BTreeSet<Position> = BTreeSet::new();

        for ln in s.lines() {
            x = 0;

            for c in ln.chars() {
                match c {
                    '>' => east_herd.insert((y, x)),
                    'v' => south_herd.insert((y, x)),
                    '.' => false,
                    _ => panic!("Unknown symbol"),
                };

                x += 1;
            }

            y += 1;
        }

        State {
            board_width: x,
            board_height: y,
            east_herd,
            south_herd,
        }
    }

    fn next(&self) -> State {
        let mut east_herd: BTreeSet<Position> = BTreeSet::new();
        let mut south_herd: BTreeSet<Position> = BTreeSet::new();

        for (y, x) in self.east_herd.iter() {
            let next_pos = (*y, (x + 1) % self.board_width);
            if !self.east_herd.contains(&next_pos) && !self.south_herd.contains(&next_pos) {
                east_herd.insert(next_pos);
            } else {
                east_herd.insert((*y, *x));
            }
        }

        for (y, x) in self.south_herd.iter() {
            let next_pos = ((y + 1) % self.board_height, *x);
            if !east_herd.contains(&next_pos) && !self.south_herd.contains(&next_pos) {
                south_herd.insert(next_pos);
            } else {
                south_herd.insert((*y, *x));
            }
        }

        State {
            board_width: self.board_width,
            board_height: self.board_height,
            east_herd,
            south_herd,
        }
    }

    #[allow(dead_code)]
    fn draw(&self) {
        for y in 0..self.board_height {
            for x in 0..self.board_width {
                if self.east_herd.contains(&(y, x)) {
                    print!(">");
                } else if self.south_herd.contains(&(y, x)) {
                    print!("v");
                } else {
                    print!(".");
                }
            }

            println!("");
        }
    }
}

pub fn ex1() -> String {
    let input_str: String = fs::read_to_string("inputs/day25.txt").expect("Could not read file");
    let mut state: State = State::from(input_str);
    let mut count: u32 = 1;

    loop {
        let next_state = state.next();

        if state == next_state {
            break;
        } else {
            count += 1;
            state = next_state;
        }
    }

    count.to_string()
}

pub fn ex2() -> String {
    String::from("N/A")
}

#[cfg(test)]
mod tests {
    use super::ex1;
    #[test]
    fn test_both_exercises() {
        assert_eq!(ex1(), "568");
    }
}
