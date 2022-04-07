use std::convert::identity;
use std::fs;

fn triad_sums(vec: Vec<u32>) -> Vec<u32> {
    vec.iter()
        .zip(vec.iter().skip(1))
        .zip(vec.iter().skip(2))
        .map(|((a, b), c)| a + b + c)
        .collect()
}

fn general_solution<F: Fn(Vec<u32>) -> Vec<u32>>(f: F) -> String {
    // loag file into a vector of numbers
    let input: String = fs::read_to_string("inputs/day1.txt").expect("Could not read file");
    let lines: Vec<u32> = input.lines().map(|s| s.parse::<u32>().unwrap()).collect();
    // perform pre-processing aggregation
    let aggregates = f(lines);
    // count cases
    aggregates
        .iter()
        .zip(aggregates.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count()
        .to_string()
}

pub fn ex1() -> String {
    // count cases using raw values
    general_solution(identity)
}

pub fn ex2() -> String {
    // count cases after getting a rolling sum of three
    general_solution(triad_sums)
}
