use regex::Regex;
use std::fs;

#[derive(Debug)]
struct TargetArea {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

#[derive(Debug)]
struct Vec2D {
    x: i32,
    y: i32,
}

impl Vec2D {
    fn inside(&self, area: &TargetArea) -> bool {
        area.min_x <= self.x && self.x <= area.max_x && area.min_y <= self.y && self.y <= area.max_y
    }

    fn incr(&mut self, v2: &Vec2D) {
        self.x += v2.x;
        self.y += v2.y;
    }

    fn slow_down(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        } else if self.x < 0 {
            self.x += 1;
        }

        self.y -= 1;
    }
}

fn load_target_area(s: String) -> TargetArea {
    let re = Regex::new(r"target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)").unwrap();
    let caps = re.captures(&s).unwrap();

    TargetArea {
        min_x: caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
        max_x: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        min_y: caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
        max_y: caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
    }
}

fn hits_target(velocity: Vec2D, area: &TargetArea) -> Option<i32> {
    let mut v = velocity;
    let mut pos = Vec2D { x: 0, y: 0 };
    let mut max_y = pos.y;

    while pos.y >= area.min_y {
        if pos.inside(area) {
            return Some(max_y);
        }

        pos.incr(&v);
        v.slow_down();
        if pos.y >= max_y {
            max_y = pos.y;
        }
    }

    None
}

fn velocity_vectors(area: &TargetArea) -> impl std::iter::Iterator<Item = Vec2D> {
    // note: I assume that the area has x coordinates positive and all y coordinates negative.
    let min_v_x: i32 = (area.min_x as f32 / 2.0).sqrt().ceil() as i32;
    let max_v_x: i32 = area.max_x;
    let min_v_y: i32 = area.min_y;
    let max_v_y: i32 = -area.min_y - 1;

    let mut v_x: i32 = min_v_x;
    let mut v_y: i32 = min_v_y;
    std::iter::from_fn(move || {
        if v_y > max_v_y {
            return None;
        }

        let ret = Vec2D { x: v_x, y: v_y };

        v_x += 1;
        if v_x > max_v_x {
            v_x = min_v_x
        }
        if v_x == min_v_x {
            v_y += 1;
        }

        Some(ret)
    })
}

pub fn ex1() -> String {
    let input_str: String = fs::read_to_string("inputs/day17.txt").expect("Could not read file");
    let target_area = load_target_area(input_str);

    velocity_vectors(&target_area)
        .map(|v| hits_target(v, &target_area))
        .filter(|opt| opt.is_some())
        .map(|opt| opt.unwrap())
        .max()
        .unwrap()
        .to_string()
}

pub fn ex2() -> String {
    let input_str: String = fs::read_to_string("inputs/day17.txt").expect("Could not read file");
    let target_area = load_target_area(input_str);

    velocity_vectors(&target_area)
        .map(|v| hits_target(v, &target_area))
        .filter(|opt| opt.is_some())
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{ex1, ex2};
    #[test]
    fn test_both_exercises() {
        assert_eq!(ex1(), "10585");
        assert_eq!(ex2(), "5247");
    }
}
