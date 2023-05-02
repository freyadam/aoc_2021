use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

type SegmentBlock = HashSet<u8>;

type LeftEncoded = [SegmentBlock; 10];
type RightEncoded = [SegmentBlock; 4];
type Solution = HashMap<u8, u8>;

const PROCESS_LINE_ERR_MSG: &'static str = "Incorrect line format";

fn process_segment_block<'a>(segment: &'a str) -> SegmentBlock {
    let mut block = SegmentBlock::new();

    for k in segment.chars().map(|c| c as usize - 'a' as usize) {
        block.insert(k as u8);
    }

    block
}

fn process_line<'a>(line: &'a str) -> (LeftEncoded, RightEncoded) {
    if let [code_str, input_str] = line.split('|').collect::<Vec<&str>>()[..2] {
        return (
            code_str
                .split_ascii_whitespace()
                .map(|s| process_segment_block(s))
                .collect::<Vec<SegmentBlock>>()
                .try_into()
                .expect(PROCESS_LINE_ERR_MSG),
            input_str
                .split_ascii_whitespace()
                .map(|s| process_segment_block(s))
                .collect::<Vec<SegmentBlock>>()
                .try_into()
                .expect(PROCESS_LINE_ERR_MSG),
        );
    }
    panic!();
}

fn solve(code: LeftEncoded) -> Solution {
    let mut solution = HashMap::new();

    // Get segment blocks for 7 and 1. 7 has length 3 while 1 has length 2. The segment that occurs in 7
    // but not in 1 identifies the top center segment.
    let seven_segment_block = code.iter().filter(|b| b.len() == 3).next().unwrap();
    let one_segment_block = code.iter().filter(|b| b.len() == 2).next().unwrap();
    let top_center_segment = seven_segment_block
        .difference(&one_segment_block)
        .next()
        .unwrap();
    solution.insert(*top_center_segment, 0);

    // Consider the upper left segment and the middle center segment for the number 4. For the group of numbers
    // with 5 segments (2, 3, and 5), upper left segment will be included once while the middle center segment
    // will be included thrice. Fill in both.
    let four_segment_block = code.iter().filter(|b| b.len() == 4).next().unwrap();
    let relevant_segments: Vec<&u8> = four_segment_block.difference(&one_segment_block).collect();
    let segment_a = *relevant_segments[0];
    let segment_b = *relevant_segments[1];
    let segment_a_count = code
        .iter()
        .filter(|b| b.len() == 5 && b.contains(&segment_a))
        .count();
    if segment_a_count == 1 {
        solution.insert(segment_a, 1);
        solution.insert(segment_b, 3);
    } else {
        solution.insert(segment_a, 3);
        solution.insert(segment_b, 1);
    }

    // For the group of numbers with 5 segments (2, 3, and 5), only 5 will contain the newly discovered
    // upper left segment. Use this information to fill-in lower right segment and then upper right segment.
    let upper_left_segment = if segment_a_count == 1 {
        segment_a
    } else {
        segment_b
    };
    let mut five_segment_block: SegmentBlock = code
        .iter()
        .cloned()
        .filter(|b| b.len() == 5 && b.contains(&upper_left_segment))
        .next()
        .unwrap();
    for k in solution.keys() {
        five_segment_block.remove(k);
    }
    let (segment_a, segment_b) = five_segment_block.iter().next_tuple().unwrap();
    let segment_a_count = code
        .iter()
        .filter(|b| b.len() == 5 && b.contains(&segment_a))
        .count();
    let mut segment_two: u8 = 0;
    let segment_five: u8;
    if segment_a_count == 2 {
        segment_five = *segment_a;
        for segment in one_segment_block {
            if *segment != *segment_a {
                segment_two = *segment;
            }
        }
    } else {
        segment_five = *segment_b;
        for segment in one_segment_block {
            if *segment != *segment_b {
                segment_two = *segment;
            }
        }
    }
    solution.insert(segment_five, 5);
    solution.insert(segment_two, 2);

    // For numbers 2 and 3, only 3 will contain lower right segment. Use this information to
    // identify bottom middle segment.
    let segment_six: u8;
    let (segment_block_a, segment_block_b): (SegmentBlock, SegmentBlock) = code
        .iter()
        .cloned()
        .filter(|b| b.len() == 5 && !b.contains(&upper_left_segment))
        .next_tuple()
        .unwrap();

    let d: HashSet<u8> = solution.keys().cloned().collect();
    if segment_block_a.contains(&segment_five) {
        segment_six = *segment_block_a.difference(&d).next().unwrap();
    } else {
        segment_six = *segment_block_b.difference(&d).next().unwrap();
    }
    solution.insert(segment_six, 6);

    // The only missing segment is the lower left one. Fill it in.
    let mut keys: HashSet<u8> = HashSet::from([0, 1, 2, 3, 4, 5, 6]);
    let mut values: HashSet<u8> = HashSet::from([0, 1, 2, 3, 4, 5, 6]);
    for (k, v) in solution.iter() {
        keys.remove(k);
        values.remove(v);
    }
    solution.insert(*keys.iter().next().unwrap(), *values.iter().next().unwrap());

    // Return a fully filled-in solution hash map.
    solution
}

fn decode(output: RightEncoded, solution: Solution) -> u32 {
    let letters: HashMap<Vec<u8>, u8> = HashMap::from([
        (vec![0, 1, 2, 4, 5, 6], 0),
        (vec![2, 5], 1),
        (vec![0, 2, 3, 4, 6], 2),
        (vec![0, 2, 3, 5, 6], 3),
        (vec![1, 2, 3, 5], 4),
        (vec![0, 1, 3, 5, 6], 5),
        (vec![0, 1, 3, 4, 5, 6], 6),
        (vec![0, 2, 5], 7),
        (vec![0, 1, 2, 3, 4, 5, 6], 8),
        (vec![0, 1, 2, 3, 5, 6], 9),
    ]);

    let decoded_output: Vec<Vec<u8>> = output
        .iter()
        .map(|b| {
            b.iter()
                .map(|k| *solution.get(k).unwrap())
                .sorted()
                .collect()
        })
        .collect();

    let mut result: u32 = 0;

    for v in decoded_output {
        result = 10 * result + *letters.get(&v).unwrap() as u32;
    }

    result
}

pub fn ex1() -> String {
    let options: HashSet<usize> = HashSet::from([2, 4, 3, 7]);
    let input_str = fs::read_to_string("inputs/day8.txt").expect("Could not read file");
    let mut sum: u32 = 0;
    for (_, output) in input_str.lines().map(|ln| process_line(ln)) {
        for block in output.iter() {
            if options.contains(&block.len()) {
                sum += 1;
            }
        }
    }

    sum.to_string()
}

pub fn ex2() -> String {
    let input_str = fs::read_to_string("inputs/day8.txt").expect("Could not read file");
    let mut solution: Solution;
    let mut sum: u32 = 0;
    for (code, output) in input_str.lines().map(|ln| process_line(ln)) {

        solution = solve(code);

        sum += decode(output, solution);
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::{ex1, ex2};
    #[test]
    fn test_both_exercises() {
        assert_eq!(ex1(), "387");
        assert_eq!(ex2(), "986034");
    }
}
