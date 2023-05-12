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
    fn is_end_state(&self) -> bool {
        let mut k: u32;
        for (c, y, x) in self.positions.iter() {
            if *y == 1 {
                return false;
            }

            k = 3 + 2 * (*c as u32 - 'A' as u32);
            if *x != k {
                return false;
            }
        }

        true
    }

    fn accessible_states(&self, all_positions: &Vec<(u32, u32)>) -> Vec<State> {
        let mut ret: Vec<State> = Vec::new();
        let mut new_positions = self.positions.clone();
        let mut c;

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

        ret
    }

    fn is_accessible(
        (c, y_old, x_old): (char, u32, u32),
        (y_new, x_new): (u32, u32),
        positions: &BTreeSet<(char, u32, u32)>,
    ) -> bool {

        // place is empty
        for (_, y, x) in positions.iter() {
            if y_old == *y && x_old == *x {
                continue;
            }

            if *y == y_new && *x == x_new {
                return false;
            }
        }

        // there exists an unblocked path in between both points
        for (_, y, x) in positions.iter() {
            if y_old == *y && x_old == *x {
                continue;
            }

            for x_temp in x_old.min(x_new)..x_old.max(x_new) {
                if *x == x_temp && *y == 1 {
                    return false;
                }
            }
            // (room-to-room case)
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
            // (room-to-hall cases)
            if y_old >= 2 && y_new == 1 {
                for y_temp in 1..y_old {
                    if *y == y_temp && *x == x_old {
                        return false;
                    }
                }
            }
            // (hall-to-room cases)
            if y_old == 1 && y_new >= 2 {
                for y_temp in 1..y_new {
                    if *y == y_temp && *x == x_new {
                        return false;
                    }
                }
            }
        }

        // the position is not in front of a chamber
        for (y, x) in [(1, 3), (1, 5), (1, 7), (1, 9)] {
            if y == y_new && x == x_new {
                return false;
            }
        }

        // movement not between two positions in a hall
        if y_old == 1 && y_new == 1 {
            return false;
        }

        // if moving to a chamber, it is the right kind
        if y_new >= 2 && x_new != 3 + 2 * (c as u32 - 'A' as u32) {
            return false;
        }

        // if moving to a chamber, no different type of amphibod is inside
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

        true
    }

    fn cost((c, y_old, x_old): (char, u32, u32), (y_new, x_new): (u32, u32)) -> u32 {
        let k: u32;

        if y_old >= 2 && y_new >= 2 {
            // room to room
            k = y_old.abs_diff(1) + y_new.abs_diff(1) + x_old.abs_diff(x_new);
        } else {
            // room to hall or hall to room
            k = y_old.abs_diff(y_new) + x_old.abs_diff(x_new);
        }

        k * (10 as u32).pow(c as u32 - 'A' as u32)
    }

}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost
            .cmp(&other.cost)
            .reverse()
            .then_with(|| self.positions.cmp(&other.positions))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn load_init_state(s: String, simplified: bool) -> State {
    let re =
        Regex::new(r"##([A-D])#([A-D])#([A-D])#([A-D])###\n  #([A-D])#([A-D])#([A-D])#([A-D])#\n  #([A-D])#([A-D])#([A-D])#([A-D])#\n  #([A-D])#([A-D])#([A-D])#([A-D])#\n")
            .unwrap();
    let caps = re.captures(&s).unwrap();
    let mut k = 0;
    
    let mut set: BTreeSet<(char, u32, u32)> = BTreeSet::new();
    for row in 0..4 {
        if simplified && (row == 1 || row == 2) {
            continue;
        }
        
        set.insert((caps.get(4 * row as usize + 1).unwrap().as_str().parse::<char>().unwrap(), k + 2, 3));
        set.insert((caps.get(4 * row as usize + 2).unwrap().as_str().parse::<char>().unwrap(), k + 2, 5));
        set.insert((caps.get(4 * row as usize + 3).unwrap().as_str().parse::<char>().unwrap(), k + 2, 7));
        set.insert((caps.get(4 * row as usize + 4).unwrap().as_str().parse::<char>().unwrap(), k + 2, 9));

        k += 1;
    }
    
    State {
        cost: 0,
        positions: set,
    }
}

fn general_solution(input_str: String, simplified: bool) -> u32 {
    let all_positions: Vec<(u32, u32)> = if simplified {
        Vec::from([(1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7),
                   (1, 8), (1, 9), (1, 10), (1, 11), (2, 3), (2, 5), (2, 7),
                   (2, 9), (3, 3), (3, 5), (3, 7), (3, 9)])
    } else {
        Vec::from([(1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7),
                   (1, 8), (1, 9), (1, 10), (1, 11), (2, 3), (2, 5), (2, 7),
                   (2, 9), (3, 3), (3, 5), (3, 7), (3, 9), (4, 3), (4, 5), (4, 7),
                   (4, 9), (5, 3), (5, 5), (5, 7), (5, 9)])
    };       
    let state = load_init_state(input_str, simplified);
    let mut best_costs: HashMap<BTreeSet<(char, u32, u32)>, u32> = HashMap::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::from([state]);

    while let Some(s) = heap.pop() {

        if s.is_end_state() {
            return s.cost;
        }

        if let Some(cost) = best_costs.get(&s.positions) {
            if cost < &s.cost {
                continue;
            }
        }

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
