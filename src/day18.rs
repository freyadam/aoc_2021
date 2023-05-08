use itertools::Itertools;
use std::fs;
use std::ops::Add;
use std::rc::Rc;

#[derive(Debug)]
enum Number {
    Literal { v: u32 },
    Pair { left: Rc<Number>, right: Rc<Number> },
}

impl Number {
    fn from(chars: &mut impl Iterator<Item = char>) -> Self {
        match chars.next() {
            Some('[') => {
                let left = Number::from(chars);
                let right = Number::from(chars);

                chars.next(); // get rid of closing ']' symbol
                return Number::Pair {
                    left: Rc::new(left),
                    right: Rc::new(right),
                };
            }
            Some(c) if '0' <= c && c <= '9' => {
                let mut s: String = String::new();
                s.push(c);

                loop {
                    let c = chars.next().unwrap();
                    if c == ',' || c == ']' {
                        return Number::Literal {
                            v: s.parse::<u32>().unwrap(),
                        };
                    }

                    s.push(c);
                }
            }
            _ => panic!("Unknown symbol"),
        }
    }

    #[allow(dead_code)]
    fn draw(&self) {
        match self {
            Number::Literal { v } => {
                print!("{v}")
            }
            Number::Pair { left, right } => {
                print!("[");
                left.draw();
                print!(",");
                right.draw();
                print!("]");
            }
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Number::Literal { v } => *v,
            Number::Pair { left, right } => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    fn add_on_edge(&self, add: u32, go_left: bool) -> Number {
        match self {
            Number::Literal { v } => Number::Literal { v: *v + add },
            Number::Pair { left, right } => Number::Pair {
                left: if go_left {
                    Rc::new(left.add_on_edge(add, go_left))
                } else {
                    left.clone()
                },
                right: if !go_left {
                    Rc::new(right.add_on_edge(add, go_left))
                } else {
                    right.clone()
                },
            },
        }
    }

    fn explode(&self, level: u16) -> Option<(Option<u32>, Number, Option<u32>)> {
        match self {
            Number::Literal { v: _ } => None,
            Number::Pair { left, right } => {
                if let (Number::Literal { v: v_left }, Number::Literal { v: v_right }) =
                    (&**left, &**right)
                {
                    if level >= 4 {
                        Some((Some(*v_left), Number::Literal { v: 0 }, Some(*v_right)))
                    } else {
                        None
                    }
                } else if let Some((lv, num, rv)) = left.explode(level + 1) {
                    Some((
                        lv,
                        Number::Pair {
                            left: Rc::new(num),
                            right: if let Some(rv_num) = rv {
                                Rc::new(right.add_on_edge(rv_num, true))
                            } else {
                                right.clone()
                            },
                        },
                        None,
                    ))
                } else if let Some((lv, num, rv)) = right.explode(level + 1) {
                    Some((
                        None,
                        Number::Pair {
                            left: if let Some(lv_num) = lv {
                                Rc::new(left.add_on_edge(lv_num, false))
                            } else {
                                left.clone()
                            },
                            right: Rc::new(num),
                        },
                        rv,
                    ))
                } else {
                    None
                }
            }
        }
    }

    fn split(&self) -> Option<Number> {
        match self {
            Number::Literal { v } => {
                if *v >= 10 {
                    Some(Number::Pair {
                        left: Rc::new(Number::Literal {
                            v: (*v as f32 / 2.0).floor() as u32,
                        }),
                        right: Rc::new(Number::Literal {
                            v: (*v as f32 / 2.0).ceil() as u32,
                        }),
                    })
                } else {
                    None
                }
            }
            Number::Pair { left, right } => {
                let left_new = left.split();
                let right_new = if left_new.is_none() {
                    right.split()
                } else {
                    None
                };

                if left_new.is_none() && right_new.is_none() {
                    return None;
                }

                Some(Number::Pair {
                    left: if left_new.is_some() {
                        Rc::new(left_new.unwrap())
                    } else {
                        left.clone()
                    },
                    right: if right_new.is_some() {
                        Rc::new(right_new.unwrap())
                    } else {
                        right.clone()
                    },
                })
            }
        }
    }

    fn simplify(self) -> Self {
        let mut current: Number = self;
        let mut changed: bool;

        loop {
            changed = false;

            if let Some((_left, new_number, _right)) = current.explode(0) {
                changed = true;
                current = new_number;
            } else if let Some(new_number) = current.split() {
                changed = true;
                current = new_number;
            }

            if !changed {
                break;
            }
        }

        current
    }
}

impl Clone for Number {
    fn clone(&self) -> Number {
        match self {
            Number::Literal { v } => Number::Literal { v: *v },
            Number::Pair { left, right } => Number::Pair {
                left: left.clone(),
                right: right.clone(),
            },
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Number::Pair {
            left: Rc::new(self),
            right: Rc::new(other),
        }
        .simplify()
    }
}

pub fn ex1() -> String {
    let input_str: String = fs::read_to_string("inputs/day18.txt").expect("Could not read file");

    input_str
        .lines()
        .map(|ln| Number::from(&mut ln.chars()))
        .reduce(|acc, n| acc + n)
        .unwrap()
        .simplify()
        .magnitude()
        .to_string()
}

pub fn ex2() -> String {
    let input_str: String = fs::read_to_string("inputs/day18.txt").expect("Could not read file");

    input_str
        .lines()
        .map(|ln| Number::from(&mut ln.chars()))
        .permutations(2)
        .map(|v| v[0].clone() + v[1].clone())
        .map(|a| a.magnitude())
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::{ex1, ex2};
    #[test]
    fn test_both_exercises() {
        assert_eq!(ex1(), "3884");
        assert_eq!(ex2(), "4595");
    }
}
