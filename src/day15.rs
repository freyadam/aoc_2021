use std::fs;

type BaseRisks = Vec<Vec<u8>>;
type TotalRisks = Vec<Vec<Option<u16>>>;

fn process_line(ln: &str) -> Vec<u8> {
    ln.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
}

fn load_base_risks_ex1(s: String) -> BaseRisks {
    s.lines().map(|ln| process_line(ln)).collect::<BaseRisks>()
}

fn load_base_risks_ex2(s: String) -> BaseRisks {
    let original_grid = load_base_risks_ex1(s);
    let mut new_grid: BaseRisks = Vec::new();

    // allocate space in the new grid
    let height = original_grid.len();
    let width = original_grid[0].len();
    let mut vec: Vec<u8>;
    for _y in 0..5 * height {
        vec = Vec::new();
        for _x in 0..5 * width {
            vec.push(0);
        }
        new_grid.push(vec);
    }

    // fill in values into the new grid
    for x_mul in 0..5 {
        for y_mul in 0..5 {
            for y in 0..height {
                for x in 0..width {
                    new_grid[y_mul * height + y][x_mul * width + x] =
                        ((original_grid[y][x] + x_mul as u8 + y_mul as u8 - 1) % 9) + 1;
                }
            }
        }
    }

    new_grid
}

fn initial_total_risks(height: usize, width: usize) -> TotalRisks {
    let mut r = TotalRisks::new();

    // create a [height x width] grid
    let mut vec: Vec<Option<u16>>;
    for _y in 0..height {
        vec = Vec::new();
        for _x in 0..width {
            vec.push(None);
        }
        r.push(vec);
    }

    // initialize the value for the starting position
    r[0][0] = Some(0);

    r
}

fn updated_val(old: Option<u16>, neighbour: Option<u16>, risk: u8) -> Option<u16> {
    if neighbour == None {
        return old;
    }

    if old == None {
        return Some(neighbour.unwrap() + risk as u16);
    }

    if old.unwrap() > neighbour.unwrap() + risk as u16 {
        return Some(neighbour.unwrap() + risk as u16);
    }

    old
}

fn update_total_risks(totals: &mut TotalRisks, base: &BaseRisks) -> bool {
    let height = totals.len();
    let width = totals[0].len();
    let mut totals_updated: bool = false;
    let mut old_val: Option<u16>;

    for y in 0..height {
        for x in 0..width {
            old_val = totals[y][x];

            if 0 < x {
                totals[y][x] = updated_val(totals[y][x], totals[y][x - 1], base[y][x]);
            }

            if x < width - 1 {
                totals[y][x] = updated_val(totals[y][x], totals[y][x + 1], base[y][x]);
            }

            if 0 < y {
                totals[y][x] = updated_val(totals[y][x], totals[y - 1][x], base[y][x]);
            }

            if y < height - 1 {
                totals[y][x] = updated_val(totals[y][x], totals[y + 1][x], base[y][x]);
            }

            if old_val != totals[y][x] {
                totals_updated = true;
            }
        }
    }
    totals_updated
}

fn general_solution(load_base_risks_fn: fn(String) -> BaseRisks) -> String {
    let input_str: String = fs::read_to_string("inputs/day15.txt").expect("Could not read file");
    let base: BaseRisks = load_base_risks_fn(input_str);

    // keep iterating and updating the board as long as at least one field is changed
    let mut total_risks: TotalRisks = initial_total_risks(base.len(), base[0].len());
    loop {
        if !update_total_risks(&mut total_risks, &base) {
            break;
        }
    }

    // return the total path risk in the lower right corner
    let vec: &Vec<Option<u16>> = &total_risks[total_risks.len() - 1];
    vec[vec.len() - 1].unwrap().to_string()
}

pub fn ex1() -> String {
    general_solution(load_base_risks_ex1)
}

pub fn ex2() -> String {
    general_solution(load_base_risks_ex2)
}

#[cfg(test)]
mod tests {
    use super::{ex1, ex2};
    #[test]
    fn test_both_exercises() {
        assert_eq!(ex1(), "503");
        assert_eq!(ex2(), "2853");
    }
}
