use std::fs;

struct Grid<T> {
    data: [T; 100],
}

impl<T: Copy> Grid<T> {
    fn get(&self, x: usize, y: usize) -> T {
        let k = y * 10 + x;

        if k >= 100 {
            panic!("Position out of range")
        }
        self.data[k]
    }

    fn set(&mut self, x: usize, y: usize, val: T) {
        let k = y * 10 + x;

        if k >= 100 {
            panic!("Position out of range")
        }
        self.data[k] = val;
    }
}

impl Grid<u8> {
    #[allow(dead_code)]
    fn draw(&self) {
        let mut iter = self.data.iter().enumerate();
        while let Some((k, value)) = iter.next() {
            print!("{}", value);
            if (k + 1) % 10 == 0 {
                println!("");
            }
        }
        println!("");
    }

    fn from(s: &String) -> Grid<u8> {
        let data: [u8; 100] = s
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        Grid { data: data }
    }
}

fn update_vicinity(grid: &mut Grid<u8>, x_size: usize, y_size: usize) {
    let x = x_size as i8;
    let y = y_size as i8;
    let arr = [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ];

    let iter = arr
        .iter()
        .filter(|(a, _)| *a >= 0)
        .filter(|(a, _)| *a <= 9)
        .filter(|(_, b)| *b >= 0)
        .filter(|(_, b)| *b <= 9);

    for (x, y) in iter {
        grid.set(
            *x as usize,
            *y as usize,
            grid.get(*x as usize, *y as usize) + 1,
        );
    }
}

fn step(grid: &mut Grid<u8>) -> u8 {
    let mut flashes: Grid<bool> = Grid { data: [false; 100] };

    // increase all energy levels by one
    for x in 0..10 {
        for y in 0..10 {
            grid.set(x, y, grid.get(x, y) + 1);
        }
    }

    // as long as there was at least one octopus with energy over a threshold in the previous cycle,
    // keep flashing
    let mut flashed: bool = true;
    while flashed {
        flashed = false;

        for x in 0..10 {
            for y in 0..10 {
                if grid.get(x, y) > 9 && !flashes.get(x, y) {
                    flashed = true;
                    flashes.set(x, y, true);
                    update_vicinity(grid, x, y);
                }
            }
        }
    }

    // reduce energy to 0 for all octopi that flashed
    for x in 0..10 {
        for y in 0..10 {
            if flashes.get(x, y) {
                grid.set(x, y, 0);
            }
        }
    }

    flashes.data.iter().filter(|b| **b).count() as u8
}

pub fn ex1() -> String {
    let input_str: String = fs::read_to_string("inputs/day11.txt").expect("Could not read file");
    let mut grid: Grid<u8> = Grid::from(&input_str);
    let mut total_flashes: u32 = 0;

    for _ in 0..100 {
        total_flashes += step(&mut grid) as u32;
    }

    total_flashes.to_string()
}

pub fn ex2() -> String {
    let input_str: String = fs::read_to_string("inputs/day11.txt").expect("Could not read file");
    let mut grid: Grid<u8> = Grid::from(&input_str);
    let mut step_count: u32 = 0;

    let mut all_flashed: bool = false;
    while !all_flashed {
        all_flashed = step(&mut grid) == 100;
        step_count += 1;
    }

    step_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::{ex1, ex2};
    #[test]
    fn test_both_exercises() {
        assert_eq!(ex1(), "1594");
        assert_eq!(ex2(), "437");
    }
}
