use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::fs;

const EX1_LIMIT: u16 = 1000;
const EX2_LIMIT: u16 = 21;
const MAX_POSITION: u16 = 10;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Game {
    pos1: u8,
    pos2: u8,
    score1: u16,
    score2: u16,
    step_count: u16,
}

impl Game {
    fn ex1_next_step(&mut self) {
        let (d1, d2, d3) = (
            ((3 * self.step_count) % 100) + 1,
            ((3 * self.step_count + 1) % 100) + 1,
            ((3 * self.step_count + 2) % 100) + 1,
        );

        if self.step_count % 2 == 0 {
            self.pos1 = (((self.pos1 as u16 + d1 + d2 + d3 - 1) % MAX_POSITION) + 1) as u8;
            self.score1 += self.pos1 as u16;
        } else {
            self.pos2 = (((self.pos2 as u16 + d1 + d2 + d3 - 1) % MAX_POSITION) + 1) as u8;
            self.score2 += self.pos2 as u16;
        }

        self.step_count += 1;
    }

    fn ex1_score(&self) -> u32 {
        3 * self.step_count as u32
            * if self.score1 >= EX1_LIMIT {
                self.score2 as u32
            } else {
                self.score1 as u32
            }
    }

    fn over(&self, limit: u16) -> bool {
        if self.score1 >= limit {
            true
        } else if self.score2 >= limit {
            true
        } else {
            false
        }
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut c: Ordering = self.score1.cmp(&other.score1);
        if !c.is_eq() {
            return c;
        }

        c = self.score2.cmp(&other.score2);
        if !c.is_eq() {
            return c;
        }

        c = self.pos1.cmp(&other.pos1);
        if !c.is_eq() {
            return c;
        }

        c = self.pos2.cmp(&other.pos2);
        if !c.is_eq() {
            return c;
        }

        self.step_count.cmp(&other.step_count)
    }
}

fn load_configuration(s: String) -> (u8, u8) {
    let re = Regex::new(r"Player 1 starting position: (\d+)\nPlayer 2 starting position: (\d+)")
        .unwrap();
    let caps = re.captures(&s).unwrap();
    (
        caps.get(1).unwrap().as_str().parse::<u8>().unwrap(),
        caps.get(2).unwrap().as_str().parse::<u8>().unwrap(),
    )
}

pub fn ex1() -> String {
    let input_str: String = fs::read_to_string("inputs/day21.txt").expect("Could not read file");
    let (pos1, pos2) = load_configuration(input_str);
    let mut game = Game {
        pos1: pos1,
        pos2: pos2,
        score1: 0,
        score2: 0,
        step_count: 0,
    };

    while !game.over(EX1_LIMIT) {
        game.ex1_next_step();
    }
    game.ex1_score().to_string()
}

pub fn ex2() -> String {
    let input_str: String = fs::read_to_string("inputs/day21.txt").expect("Could not read file");
    let (pos1, pos2) = load_configuration(input_str);
    let mut wins1: i64 = 0;
    let mut wins2: i64 = 0;
    let start_game = Game {
        pos1,
        pos2,
        score1: 0,
        score2: 0,
        step_count: 0,
    };
    // Set of game configurations to be processed from the earliest
    // (namely the lowest score) towards the "later" ones. Initialized with
    // the starting game state.
    let mut to_process: BTreeSet<Game> = BTreeSet::from([start_game.clone()]);
    // How many time was given game configuration reached?
    // Starting game state can be reached only in one way, so its count is 1.
    let mut counts: HashMap<Game, i64> = HashMap::from([(start_game, 1)]);

    let mut c: i64;
    let mut pos: u8;
    let mut next_game: Game;
    let mut game_count: i64;
    while let Some(game) = to_process.pop_first() {
        // Games that were already won are not continued. Instead their count is
        // added to the particular player's win counter.
        if game.over(EX2_LIMIT) {
            c = *counts.get(&game).unwrap();
            if game.score1 >= EX2_LIMIT {
                wins1 += c;
            } else {
                wins2 += c;
            }
            continue;
        }

        // For non-winning games, all possible follow-ups are considered and added to
        // `to_process` and `counts`. If a `counts` record already exists, it is
        // incremented.
        for (mv, count) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
            if game.step_count % 2 == 0 {
                pos = (((game.pos1 as u16 + mv - 1) % MAX_POSITION) + 1) as u8;
                next_game = Game {
                    pos1: pos,
                    pos2: game.pos2,
                    score1: game.score1 + pos as u16,
                    score2: game.score2,
                    step_count: game.step_count + 1,
                };
                game_count = *counts.get(&game).unwrap();
                let entry = counts.entry(next_game.clone()).or_default();
                *entry += count * game_count;
                to_process.insert(next_game);
            } else {
                pos = (((game.pos2 as u16 + mv - 1) % MAX_POSITION) + 1) as u8;
                next_game = Game {
                    pos1: game.pos1,
                    pos2: pos,
                    score1: game.score1,
                    score2: game.score2 + pos as u16,
                    step_count: game.step_count + 1,
                };
                game_count = *counts.get(&game).unwrap();
                let entry = counts.entry(next_game.clone()).or_default();
                *entry += count * game_count;
                to_process.insert(next_game);
            }
        }
    }

    wins1.max(wins2).to_string()
}

#[cfg(test)]
mod tests {
    use super::{ex1, ex2};
    #[test]
    fn test_both_exercises() {
        assert_eq!(ex1(), "998088");
        assert_eq!(ex2(), "306621346123766");
    }
}
