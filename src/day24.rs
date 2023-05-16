use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct IterationParams {
    add: i32,
    add2: i32,
    div: i32,
}

fn load_configurations(s: String) -> Vec<IterationParams> {
    let mut ret = Vec::new();
    let mut lines = s.lines().clone();
    let mut add;
    let mut add2;
    let mut div;
    let mut caps;
    let mut ln;
    let re = Regex::new(r"(add|div) [xyz] (-?\d+)").unwrap();

    // Process individual instruction sequences starting with "inp w".
    // Each iteration has three variable values. Each struct will contain
    // this trinity.
    while let Some("inp w") = lines.next() {
        lines.nth(2);

        ln = lines.next().unwrap();
        caps = re.captures(&ln).unwrap();
        div = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();

        ln = lines.next().unwrap();
        caps = re.captures(&ln).unwrap();
        add = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();

        lines.nth(8);

        ln = lines.next().unwrap();
        caps = re.captures(&ln).unwrap();
        add2 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();

        lines.nth(1);

        ret.push(IterationParams { add, add2, div });
    }

    ret
}

fn iteration(w: i32, z: i32, params: &IterationParams) -> i32 {
    // This is a function extracted from the input instructions.
    // Only changing parameters are separated into the `IterationParams`
    // as everything else stays the same.
    if w == (z % 26) + params.add {
        z / params.div
    } else {
        26 * z / params.div + w + params.add2
    }
}

fn general_solution(
    param_sequence: Vec<IterationParams>,
    selection_fn: fn(u128, u128) -> u128,
) -> u128 {
    let mut map: HashMap<i32, u128> = HashMap::from([(0, 0)]);
    let mut temp_map: HashMap<i32, u128>;
    let mut contraction_counts: [u8; 14] = [0; 14];

    // Precompute the number of contractions that will be performed in the future for each iteration.
    // These values are used for setting upper and lower bounds.
    if param_sequence[13].div == 26 {
        contraction_counts[13] = 1;
    }
    for k in (0..param_sequence.len() - 1).rev() {
        contraction_counts[k] =
            contraction_counts[k + 1] + if param_sequence[k].div == 26 { 1 } else { 0 };
    }

    for (k, params) in param_sequence.iter().enumerate() {
        temp_map = HashMap::new();

        for (z, ext) in map.iter() {
            // As each iteration the value can be divided by 26 or not at all,
            // some values are too large or too small to realistically reach 0. These can be pruned.
            let r = (26 as u32).pow(contraction_counts[k] as u32) as i64;
            if *z as i64 > r {
                continue;
            }
            let r = -((26 as u32).pow(k as u32 - contraction_counts[k] as u32) as i64);
            if (*z as i64) < r {
                continue;
            }

            for w in 1..10 {
                // Compute value of the 'z' register for the next iteration.
                let z_next = iteration(w, *z, params);

                // Insert into the map if the new value of register z is not there yet or
                // if the aggregated value can be improved.
                let v_new = 10 * ext + w as u128;
                if temp_map.contains_key(&z_next) {
                    temp_map
                        .entry(z_next)
                        .and_modify(|v| *v = selection_fn(*v, v_new));
                } else {
                    temp_map.insert(z_next, v_new);
                }
            }
        }

        map = temp_map;
    }

    *map.get(&0).unwrap()
}

pub fn ex1() -> String {
    let input_str: String = fs::read_to_string("inputs/day24.txt").expect("Could not read file");
    let param_sequence = load_configurations(input_str);

    general_solution(param_sequence, |a, b| a.max(b)).to_string()
}

pub fn ex2() -> String {
    let input_str: String = fs::read_to_string("inputs/day24.txt").expect("Could not read file");
    let param_sequence = load_configurations(input_str);

    general_solution(param_sequence, |a, b| a.min(b)).to_string()
}

#[cfg(test)]
mod tests {
    use super::{ex1, ex2};
    #[test]
    fn test_both_exercises() {
        assert_eq!(ex1(), "99893999291967");
        assert_eq!(ex2(), "34171911181211");
    }
}
