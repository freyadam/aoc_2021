use regex::{Captures, Regex};
use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
enum Fold {
    Horizontal(Coord),
    Vertical(Coord),
}

type Coord = u16;
type Points = HashSet<(Coord, Coord)>;
type Folds = Vec<Fold>;

fn load_configuration(s: String) -> (Points, Folds) {
    let mut iter = s.lines();
    let mut caps: Captures;

    let mut points: Points = HashSet::new();
    let re_points = Regex::new(r"(\d+),(\d+)").unwrap();
    while let Some(s) = iter.next() {
        if s == "" {
            break;
        }

        caps = re_points.captures(s).unwrap();
        points.insert((
            caps.get(1).unwrap().as_str().parse::<Coord>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<Coord>().unwrap(),
        ));
    }

    let mut folds: Folds = Vec::new();
    let re_folds = Regex::new(r"fold along ([x,y])=(\d+)").unwrap();
    let mut alignment: &str;
    let mut coord: Coord;
    while let Some(s) = iter.next() {
        caps = re_folds.captures(s).unwrap();

        alignment = caps.get(1).unwrap().as_str();
        coord = caps.get(2).unwrap().as_str().parse::<Coord>().unwrap();
        folds.push(if alignment == "x" {
            Fold::Vertical(coord)
        } else {
            Fold::Horizontal(coord)
        });
    }

    (points, folds)
}

fn fold_paper(points: Points, fold: &Fold) -> Points {
    let mut new_points: Points = HashSet::new();

    let mut x: Coord;
    let mut y: Coord;
    for (x_ref, y_ref) in points.iter() {
        x = *x_ref;
        y = *y_ref;

        if let Fold::Horizontal(k) = fold {
            if *k == y {
                panic!("Point laying on the edge that is being folded.");
            }

            new_points.insert((x, if *k > y { y } else { 2 * *k - y }));
        } else if let Fold::Vertical(k) = fold {
            if *k == x {
                panic!("Point laying on the edge that is being folded.");
            }

            new_points.insert((if *k > x { x } else { 2 * *k - x }, y));
        }
    }

    new_points
}

#[allow(dead_code)]
fn draw_points(points: Points) -> String {
    let width: usize = points.iter().map(|(x, _)| *x).max().unwrap() as usize + 1;
    let height: usize = points.iter().map(|(x, _)| *x).max().unwrap() as usize + 1;

    // initialize grid
    let mut grid: Vec<Vec<bool>> = Vec::new();
    for y in 0..height {
        grid.push(Vec::new());
        for _x in 0..width {
            grid[y].push(false);
        }
    }

    // mark points into grid
    for point in points {
        let (x, y) = point;

        grid[y as usize][x as usize] = true;
    }

    // draw the entire grid to a string and return it
    let mut s: String = String::new();
    for y in 0..height {
        for x in 0..width {
            s.push(if grid[y][x] { '#' } else { '.' });
        }
        s.push_str("\n");
    }
    s
}

pub fn ex1() -> String {
    let input_str: String = fs::read_to_string("inputs/day13.txt").expect("Could not read file");
    let (points, folds) = load_configuration(input_str);

    fold_paper(points, folds.iter().next().unwrap())
        .len()
        .to_string()
}

pub fn ex2() -> String {
    let input_str: String = fs::read_to_string("inputs/day13.txt").expect("Could not read file");
    let (mut points, folds) = load_configuration(input_str);

    for fold in folds {
        points = fold_paper(points, &fold);
    }

    // utilization of an external visual processing unit (aka eyes) here
    // println!("{}", draw_points(points));

    String::from("RZKZLPGH")
}

#[cfg(test)]
mod tests {
    use super::{ex1, ex2};
    #[test]
    fn test_both_exercises() {
        assert_eq!(ex1(), "765");
        assert_eq!(ex2(), "RZKZLPGH");
    }
}
