use itertools::Itertools;
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::fs;

type Rules = HashMap<(char, char), char>;
type PairCounts = HashMap<(char, char), u64>;

fn load_configuration(s: String) -> (String, Rules) {
    let mut lines = s.lines();

    // load starting polymer configuration
    let polymer = String::from(lines.next().unwrap());

    // skip empty line
    lines.next();

    // load pair insertion rules
    let mut rules = HashMap::new();
    let mut caps: Captures;
    let re_rule = Regex::new(r"([A-Z])([A-Z]) -> ([A-Z])").unwrap();
    while let Some(rule_str) = lines.next() {
        caps = re_rule.captures(rule_str).unwrap();
        rules.insert(
            (
                caps.get(1).unwrap().as_str().parse::<char>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<char>().unwrap(),
            ),
            caps.get(3).unwrap().as_str().parse::<char>().unwrap(),
        );
    }

    (polymer, rules)
}

fn original_pair_counts(s: &str) -> PairCounts {
    let mut map = HashMap::new();

    for tpl in s.chars().zip(s.chars().skip(1)) {
        *map.entry(tpl).or_default() += 1;
    }

    map
}

fn next_step(counts: PairCounts, rules: &Rules) -> PairCounts {
    let mut ret = PairCounts::new();
    let mut b: char;

    for ((a, c), count) in counts {
        b = *rules.get(&(a, c)).unwrap();
        *ret.entry((a, b)).or_default() += count;
        *ret.entry((b, c)).or_default() += count;
    }
    ret
}

fn occurence_counts(pair_counts: PairCounts, s: &str) -> HashMap<char, u64> {
    let mut map: HashMap<char, u64> = HashMap::new();

    for ((c1, c2), count) in pair_counts {
        *map.entry(c1).or_default() += count;
        *map.entry(c2).or_default() += count;
    }

    *map.entry(s.chars().next().unwrap()).or_default() += 1;
    *map.entry(s.chars().last().unwrap()).or_default() += 1;

    for (_, count) in map.iter_mut() {
        *count /= 2;
    }
    map
}

fn general_solution(step_count: u16) -> String {
    let input_str: String = fs::read_to_string("inputs/day14.txt").expect("Could not read file");
    let (s, rules) = load_configuration(input_str);
    let mut pair_counts = original_pair_counts(&s);

    for _ in 0..step_count {
        pair_counts = next_step(pair_counts, &rules);
    }

    let counts = occurence_counts(pair_counts, &s);
    let mut occurence_iter = counts
        .iter()
        .map(|(_, v)| v)
        .sorted_by(|v1, v2| Ord::cmp(&v1, &v2));
    let first = occurence_iter.next().unwrap();
    let last = occurence_iter.last().unwrap();
    (last - first).to_string()
}

pub fn ex1() -> String {
    general_solution(10)
}

pub fn ex2() -> String {
    general_solution(40)
}

#[cfg(test)]
mod tests {
    use super::{ex1, ex2};
    #[test]
    fn test_both_exercises() {
        assert_eq!(ex1(), "2112");
        assert_eq!(ex2(), "3243771149914");
    }
}
