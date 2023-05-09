use std::collections::HashSet;
use std::fs;

type Positions = HashSet<(i32, i32)>;
type EnhancementMap = [bool; 512];
type Pixels = HashSet<(i32, i32, bool)>;

struct Image {
    pixels: Pixels,
    is_background_white: bool,
}

fn retrieve_configuration(s: String) -> (EnhancementMap, Image) {
    let v: Vec<&str> = s.split("\n\n").collect();
    let mut map: EnhancementMap = [false; 512];
    for (k, c) in v[0].chars().filter(|c| *c != '\n').enumerate() {
        map[k] = c == '#';
    }

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut pixels: Pixels = Pixels::new();
    for c in v[1].chars() {
        match c {
            '\n' => {
                x = 0;
                y += 1;
            }
            '.' => {
                pixels.insert((y, x, false));
                x += 1;
            }
            '#' => {
                pixels.insert((y, x, true));
                x += 1;
            }
            _ => {
                panic!("Unexpected symbol");
            }
        }
    }

    (
        map,
        Image {
            pixels: pixels,
            is_background_white: false,
        },
    )
}

fn is_white((y, x): (i32, i32), img: &Image, map: &EnhancementMap) -> bool {
    let mut k: usize = 0;

    for (inc_y, inc_x) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        k = 2 * k
            + if img.pixels.get(&(y + inc_y, x + inc_x, true)).is_some() {
                1
            } else if img.pixels.get(&(y + inc_y, x + inc_x, false)).is_some() {
                0
            } else if img.is_background_white {
                1
            } else {
                0
            }
    }

    map[k]
}

fn surrounding_candidates((y, x): (i32, i32)) -> Positions {
    let mut candidates = Positions::new();

    for (inc_y, inc_x) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        candidates.insert((y + inc_y, x + inc_x));
    }

    candidates
}

fn step(img: Image, map: &EnhancementMap) -> Image {
    let mut new_pixels = Pixels::new();
    let mut candidates = Positions::new();

    for p in img
        .pixels
        .iter()
        .map(|(y, x, _)| surrounding_candidates((*y, *x)))
        .flatten()
    {
        candidates.insert(p);
    }

    for (y, x) in candidates {
        new_pixels.insert((y, x, is_white((y, x), &img, &map)));
    }

    Image {
        pixels: new_pixels,
        is_background_white: if img.is_background_white {
            map[511]
        } else {
            map[0]
        },
    }
}

pub fn ex1() -> String {
    let input_str: String = fs::read_to_string("inputs/day20.txt").expect("Could not read file");
    let (map, mut image) = retrieve_configuration(input_str);

    for _ in 0..2 {
        image = step(image, &map);
    }

    image
        .pixels
        .iter()
        .filter(|(_, _, c)| *c)
        .count()
        .to_string()
}

pub fn ex2() -> String {
    let input_str: String = fs::read_to_string("inputs/day20.txt").expect("Could not read file");
    let (map, mut image) = retrieve_configuration(input_str);

    for _ in 0..50 {
        image = step(image, &map);
    }

    image
        .pixels
        .iter()
        .filter(|(_, _, c)| *c)
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{ex1, ex2};
    #[test]
    fn test_both_exercises() {
        assert_eq!(ex1(), "5218");
        assert_eq!(ex2(), "15527");
    }
}
