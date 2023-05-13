use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap, HashMap};
use std::fs;

#[derive(Debug, Eq, PartialEq)]
struct State {
    cost: u32,
    positions: BTreeSet<(char, u32, u32)>,
}

impl State {
    fn new(s: String, simplified: bool) -> State {
        let mut re_str: String = String::from("##([A-D])#([A-D])#([A-D])#([A-D])###\n");
        for _ in 0..3 {
            re_str.push_str("  #([A-D])#([A-D])#([A-D])#([A-D])#\n");
        }
        let re = Regex::new(&re_str).unwrap();
        let caps = re.captures(&s).unwrap();

        let mut y_counter = 0;
        let mut set: BTreeSet<(char, u32, u32)> = BTreeSet::new();
        for row in 0..4 {
            if simplified && (row == 1 || row == 2) {
                continue;
            }

            set.insert((caps.get(4 * row as usize + 1).unwrap().as_str().parse::<char>().unwrap(), y_counter + 2, 3));
            set.insert((caps.get(4 * row as usize + 2).unwrap().as_str().parse::<char>().unwrap(), y_counter + 2, 5));
            set.insert((caps.get(4 * row as usize + 3).unwrap().as_str().parse::<char>().unwrap(), y_counter + 2, 7));
            set.insert((caps.get(4 * row as usize + 4).unwrap().as_str().parse::<char>().unwrap(), y_counter + 2, 9));

            y_counter += 1;
        }

        State {
            cost: 0,
            positions: set,
        }
    }

    fn amphipod_line(c: char) -> u32 {
        3 + 2 * (c as u32 - 'A' as u32)
    }

    fn amphipod_cost(c: char) -> u32 {
        (10 as u32).pow(c as u32 - 'A' as u32)
    }

    fn is_end_state(&self) -> bool {
        // Check that each amphipod is in the correct chamber.
        for (c, y, x) in self.positions.iter() {
            if *y == 1 {
                return false;
            }

            if *x != State::amphipod_line(*c) {
                return false;
            }
        }

        true
    }

    fn accessible_states(&self, all_positions: &Vec<(u32, u32)>) -> Vec<State> {
        let mut ret: Vec<State> = Vec::new();
        let mut new_positions = self.positions.clone();
        let mut c;

        // For each amphipod and every position on the board, check that this position is reachable.
        // If it is, create a new state for it.
        for old_pos in self.positions.iter() {
            c = old_pos.0;

            for (y_new, x_new) in all_positions {
                if !State::is_accessible(*old_pos, (*y_new, *x_new), &self.positions) {
                    continue;
                }

                new_positions.remove(old_pos);
                new_positions.insert((c, *y_new, *x_new));                
                ret.push(State {
                    cost: self.cost + State::cost(*old_pos, (*y_new, *x_new)),
                    positions: new_positions.clone(),
                });
                new_positions.remove(&(c, *y_new, *x_new));
                new_positions.insert(*old_pos);
            }

        }

        // Return all states where a single amphipod moved (possibly multiple spaces).
        ret
    }

    fn is_accessible(
        (c, y_old, x_old): (char, u32, u32),
        (y_new, x_new): (u32, u32),
        positions: &BTreeSet<(char, u32, u32)>,
    ) -> bool {

        // Accessed point is free of other amphipods.
        for (_, y, x) in positions.iter() {
            if y_old == *y && x_old == *x {
                continue;
            }

            if *y == y_new && *x == x_new {
                return false;
            }
        }

        // There exists an unblocked path in between both points.
        for (_, y, x) in positions.iter() {
            if y_old == *y && x_old == *x {
                continue;
            }

            for x_temp in x_old.min(x_new)..x_old.max(x_new) {
                if *x == x_temp && *y == 1 {
                    return false;
                }
            }
            // (room-to-room case only)
            if y_old >= 2 && y_new >= 2 {
                for y_temp in 1..y_old {
                    if *y == y_temp && *x == x_old {
                        return false;
                    }
                }
                for y_temp in 1..y_new {
                    if *y == y_temp && *x == x_new {
                        return false;
                    }
                }
            }

            // (room-to-hall cases only)
            if y_old >= 2 && y_new == 1 {
                for y_temp in 1..y_old {
                    if *y == y_temp && *x == x_old {
                        return false;
                    }
                }
            }
            // (hall-to-room cases only)
            if y_old == 1 && y_new >= 2 {
                for y_temp in 1..y_new {
                    if *y == y_temp && *x == x_new {
                        return false;
                    }
                }
            }
        }

        // The position is not in front of a chamber which can never contain
        // any amphipods.
        for (y, x) in [(1, 3), (1, 5), (1, 7), (1, 9)] {
            if y == y_new && x == x_new {
                return false;
            }
        }

        // There can be no movement between two places in the hall.
        if y_old == 1 && y_new == 1 {
            return false;
        }

        // Each kind of amphipod can move only to its own chamber.
        if y_new >= 2 && x_new != State::amphipod_line(c) {
            return false;
        }

        // If moving to a chamber, no different type of amphibod is inside.
        if y_new >= 2 {
            for (c_other, y, x) in positions.iter() {
                if *x != x_new {
                    continue;
                }

                if *y == 1 {
                    continue;
                }

                if c != *c_other {
                    return false;
                }
            }
        }

        // If all conditions were passed, the new position can be accessed.
        true
    }

    fn cost((c, y_old, x_old): (char, u32, u32), (y_new, x_new): (u32, u32)) -> u32 {
        let k: u32;

        if y_old >= 2 && y_new >= 2 {
            // Room-to-room movement
            k = y_old.abs_diff(1) + y_new.abs_diff(1) + x_old.abs_diff(x_new);
        } else {
            // Room-to-hall or hall-to-room movement
            k = y_old.abs_diff(y_new) + x_old.abs_diff(x_new);
        }

        // The cost depends on the distance and the type of the amphipod.
        k * State::amphipod_cost(c)
    }

}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost
            .cmp(&other.cost)
            .reverse() // As a max-heap is used, it is necessary to reverse the cost comparison here
            .then_with(|| self.positions.cmp(&other.positions))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn general_solution(input_str: String, simplified: bool) -> u32 {
    let mut all_positions: Vec<(u32, u32)> = Vec::from([(1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (1, 6),
                                                        (1, 7), (1, 8), (1, 9), (1, 10), (1, 11),
                                                        (2, 3), (2, 5), (2, 7), (2, 9),
                                                        (3, 3), (3, 5), (3, 7), (3, 9)]);
    // Add two more lines for the more complex version of the puzzle.
    if !simplified {
        all_positions.append(&mut Vec::from([(4, 3), (4, 5), (4, 7), (4, 9),
                                             (5, 3), (5, 5), (5, 7), (5, 9)]));
    }

    // Initialize data structures needed for Dijkstra.
    let mut best_costs: HashMap<BTreeSet<(char, u32, u32)>, u32> = HashMap::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::from([State::new(input_str, simplified)]);

    while let Some(s) = heap.pop() {
        // Return cost if a solution was found.
        if s.is_end_state() {
            return s.cost;
        }
        // Skip state if a better solution was already found.
        if let Some(cost) = best_costs.get(&s.positions) {
            if cost < &s.cost {
                continue;
            }
        }
        // Extend heap with newly accessible states.
        for accessible_s in s.accessible_states(&all_positions) {
            match best_costs.get(&accessible_s.positions) {
                Some(current_cost) if accessible_s.cost >= *current_cost => {}
                _ => {
                    best_costs.insert(accessible_s.positions.clone(), accessible_s.cost);
                    heap.push(accessible_s);
                },
            }
        }
    }

    panic!("No solution was found.")    
}

pub fn ex1() -> String {
    let input_str: String = fs::read_to_string("inputs/day23.txt").expect("Could not read file");
    general_solution(input_str, true).to_string()
}

pub fn ex2() -> String {
    let input_str: String = fs::read_to_string("inputs/day23.txt").expect("Could not read file");
    general_solution(input_str, false).to_string()
}

#[cfg(test)]
mod tests {
    use super::{ex1, ex2};
    #[test]
    fn test_both_exercises() {
        assert_eq!(ex1(), "14415");
        assert_eq!(ex2(), "41121");
    }
}
