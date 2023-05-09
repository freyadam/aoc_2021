use itertools::Itertools;
use ndarray::prelude::*;
use ndarray::{Array, Ix1, Ix2};
use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::fs;
use std::ops::Add;

type Vec3D = Array<i32, Ix1>;
type Mtx = Array<i32, Ix2>;

type BeaconSignals = HashSet<Vec3D>;

#[derive(Debug)]
struct RelativePosition {
    translation: Vec3D,
    rotation: Mtx,
}

impl RelativePosition {
    fn manhattan_dist(&self, other: &Self) -> i32 {
        (self.translation[0] - other.translation[0]).abs()
            + (self.translation[1] - other.translation[1]).abs()
            + (self.translation[2] - other.translation[2]).abs()
    }
}

impl Add for RelativePosition {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        RelativePosition {
            translation: change_basis(&self, other.translation),
            rotation: self.rotation.dot(&other.rotation),
        }
    }
}

fn all_possible_rotations() -> Vec<Mtx> {
    let orders: Vec<Mtx> = Vec::from([
        array![[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        array![[1, 0, 0], [0, 0, 1], [0, 1, 0]],
        array![[0, 1, 0], [1, 0, 0], [0, 0, 1]],
        array![[0, 1, 0], [0, 0, 1], [1, 0, 0]],
        array![[0, 0, 1], [1, 0, 0], [0, 1, 0]],
        array![[0, 0, 1], [0, 1, 0], [1, 0, 0]],
    ]);
    let signs: Vec<Vec3D> = Vec::from([
        array![1, 1, 1],
        array![1, 1, -1],
        array![1, -1, 1],
        array![1, -1, -1],
        array![-1, 1, 1],
        array![-1, 1, -1],
        array![-1, -1, 1],
        array![-1, -1, -1],
    ]);

    orders
        .iter()
        .cartesian_product(signs.iter())
        .map(|(o, s)| o * s)
        .collect()
}

fn load_scanner_configurations(s: String) -> Vec<BeaconSignals> {
    let mut result: Vec<BeaconSignals> = Vec::new();
    let mut current: BeaconSignals = BeaconSignals::new();
    let mut iter = s.lines().filter(|ln| *ln != "");
    let header_re = Regex::new(r"^--- scanner \d+ ---$").unwrap();
    let coords_re = Regex::new(r"^(-?\d+),(-?\d+),(-?\d+)$").unwrap();

    while let Some(ln) = iter.next() {
        if let Some(_) = header_re.captures(ln) {
            if current.len() > 0 {
                result.push(current.clone());
            }
            current = BeaconSignals::new();
        } else if let Some(caps) = coords_re.captures(ln) {
            current.insert(array![
                caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            ]);
        }
    }

    if current.len() > 0 {
        result.push(current);
    }

    result
}

fn relative_scanner_position(
    signals_a: &BeaconSignals,
    signals_b: &BeaconSignals,
    all_rots: &Vec<Mtx>,
) -> Option<RelativePosition> {
    let it = signals_a
        .iter()
        .skip(11)
        .cartesian_product(signals_b.iter().skip(11))
        .cartesian_product(all_rots)
        .map(|((a, b), c)| (a, b, c));

    for (v1, v2, rot) in it {
        let v: Vec3D = v1
            - rot
                .dot(&v2.clone().into_shape((3, 1)).unwrap())
                .into_shape(3)
                .unwrap();
        let pos = RelativePosition {
            rotation: rot.clone(),
            translation: v,
        };

        if signals_a
            .intersection(&change_basis_for_all(&pos, signals_b.clone()))
            .count()
            >= 12
        {
            return Some(pos);
        }
    }

    None
}

fn change_basis(pos: &RelativePosition, v: Vec3D) -> Vec3D {
    &pos.rotation
        .dot(&v.into_shape((3, 1)).unwrap())
        .into_shape(3)
        .unwrap()
        + &pos.translation
}

fn change_basis_for_all(pos: &RelativePosition, signals: BeaconSignals) -> BeaconSignals {
    signals
        .iter()
        .map(|s| change_basis(&pos, s.clone()))
        .collect()
}

fn general_solution(
    signal_sets: Vec<BeaconSignals>,
) -> (Vec<BeaconSignals>, Vec<RelativePosition>) {
    let mut signals: Vec<BeaconSignals> = signal_sets;
    let mut positions: Vec<RelativePosition> = Vec::new();
    let mut todo: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let all_rots = all_possible_rotations();

    for _ in 0..signals.len() {
        positions.push(RelativePosition {
            translation: array![0, 0, 0],
            rotation: array![[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        })
    }

    for j in 1..signals.len() {
        todo.insert(j);
        queue.push_back((0, j));
    }

    while let Some((i, j)) = queue.pop_front() {
        if !todo.contains(&j) {
            continue;
        }
        if let Some(pos) = relative_scanner_position(&signals[i], &signals[j], &all_rots) {
            signals[j] = change_basis_for_all(&pos, signals[j].clone());
            todo.remove(&j);
            for k in todo.iter() {
                queue.push_back((j, *k));
            }
            positions[j] = pos;
        }
    }
    assert!(todo.is_empty());

    (signals, positions)
}

pub fn ex1() -> String {
    let input_str: String = fs::read_to_string("inputs/day19.txt").expect("Could not read file");
    let scanner_confs: Vec<BeaconSignals> = load_scanner_configurations(input_str);

    let (signals, _) = general_solution(scanner_confs);
    let mut res: BeaconSignals = BeaconSignals::new();
    for bs in signals {
        res.extend(bs);
    }
    res.len().to_string()
}

pub fn ex2() -> String {
    let input_str: String = fs::read_to_string("inputs/day19.txt").expect("Could not read file");
    let scanner_confs: Vec<BeaconSignals> = load_scanner_configurations(input_str);

    let (_, positions) = general_solution(scanner_confs);
    positions
        .iter()
        .cartesian_product(positions.iter())
        .map(|(a, b)| a.manhattan_dist(b))
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{ex1, ex2};
    #[test]
    fn test_both_exercises() {
        assert_eq!(ex1(), "335");
        assert_eq!(ex2(), "10864");
    }
}
