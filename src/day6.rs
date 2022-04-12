use std::fs;

type LanternPopulation = [u64; 9];

fn load_population(definition: &str) -> LanternPopulation {
    let mut population: LanternPopulation = [0; 9];

    for fish in definition
        .split(',')
        .map(|s| s.parse::<usize>().expect("Incorrect definition format"))
    {
        population[fish] += 1;
    }

    population
}

fn next_day(population: LanternPopulation) -> LanternPopulation {
    [
        population[1],
        population[2],
        population[3],
        population[4],
        population[5],
        population[6],
        population[7] + population[0],
        population[8],
        population[0],
    ]
}

fn population_total(population: LanternPopulation) -> u64 {
    population.iter().sum()
}

fn general_solution(days: u16) -> String {
    // initialize state
    let mut input_str = fs::read_to_string("inputs/day6.txt").expect("Could not read file");
    input_str.pop(); // remove newline
    let mut population = load_population(&input_str[..]);
    // iterate
    for _ in 0..days {
        population = next_day(population);
    }
    // return sum as string
    population_total(population).to_string()
}

pub fn ex1() -> String {
    general_solution(80)
}

pub fn ex2() -> String {
    general_solution(256)
}
