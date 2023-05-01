use std::fs;

fn ex1_loss(positions: &Vec<i32>, current: i32) -> i32 {
    positions.iter().map(|p| (current - p).abs()).sum()
}

fn ex2_loss(positions: &Vec<i32>, current: i32) -> i32 {
    fn f(k: i32) -> i32 {
        let half: f32 = k as f32 / 2.0;
        (half.floor() * (k + 1) as f32 + half.ceil() * (k % 2) as f32) as i32
    }

    positions.iter().map(|p| f((current - p).abs())).sum()
}

fn load_positions(definition: &str) -> Vec<i32> {
    definition
        .split(',')
        .map(|s| s.parse::<i32>().expect("Incorrect definition format"))
        .collect()
}

fn general_solution(loss_fn: fn(&Vec<i32>, i32) -> i32) -> String {
    // initialize state
    let mut input_str = fs::read_to_string("inputs/day7.txt").expect("Could not read file");
    input_str.pop(); // remove newline
    let positions = load_positions(&input_str[..]);

    let mut current: i32 = positions.iter().sum::<i32>() / positions.iter().count() as i32;
    let mut current_loss = loss_fn(&positions, current);
    let sgn: i32 = if loss_fn(&positions, current - 1) < loss_fn(&positions, current) {
        -1
    } else {
        1
    };

    let mut new_loss = loss_fn(&positions, current + sgn);
    while new_loss < current_loss {
        current += sgn;
        current_loss = new_loss;
        new_loss = loss_fn(&positions, current + sgn);
    }

    loss_fn(&positions, current).to_string()
}

pub fn ex1() -> String {
    general_solution(ex1_loss)
}
pub fn ex2() -> String {
    general_solution(ex2_loss)
}

#[cfg(test)]
mod tests {
    use super::{ex1, ex2};
    #[test]
    fn test_both_exercises() {
        assert_eq!(ex1(), "323647");
        assert_eq!(ex2(), "87640209");
    }
}
