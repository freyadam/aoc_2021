use regex::Regex;
use std::collections::VecDeque;
use std::fs;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Cuboid {
    x_from: i32,
    x_to: i32,
    y_from: i32,
    y_to: i32,
    z_from: i32,
    z_to: i32,
}

#[derive(Debug)]
struct RebootStep {
    cuboid: Cuboid,
    on: bool,
}

impl Cuboid {
    fn intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        let no_intersection_x = self.x_to <= other.x_from || other.x_to <= self.x_from;
        let no_intersection_y = self.y_to <= other.y_from || other.y_to <= self.y_from;
        let no_intersection_z = self.z_to <= other.z_from || other.z_to <= self.z_from;

        if no_intersection_x || no_intersection_y || no_intersection_z {
            return None;
        }

        Some(Cuboid {
            x_from: self.x_from.max(other.x_from),
            x_to: self.x_to.min(other.x_to),
            y_from: self.y_from.max(other.y_from),
            y_to: self.y_to.min(other.y_to),
            z_from: self.z_from.max(other.z_from),
            z_to: self.z_to.min(other.z_to),
        })
    }

    fn dissasemble(&self, subpiece: &Cuboid) -> Vec<Cuboid> {
        let x_points = [self.x_from, subpiece.x_from, subpiece.x_to, self.x_to];
        let y_points = [self.y_from, subpiece.y_from, subpiece.y_to, self.y_to];
        let z_points = [self.z_from, subpiece.z_from, subpiece.z_to, self.z_to];
        let mut ret: Vec<Cuboid> = Vec::new();

        for (x_from, x_to) in x_points.iter().zip(x_points.iter().skip(1)) {
            for (y_from, y_to) in y_points.iter().zip(y_points.iter().skip(1)) {
                for (z_from, z_to) in z_points.iter().zip(z_points.iter().skip(1)) {
                    if x_from == x_to {
                        continue;
                    }

                    if y_from == y_to {
                        continue;
                    }

                    if z_from == z_to {
                        continue;
                    }

                    ret.push(Cuboid {
                        x_from: *x_from,
                        x_to: *x_to,
                        y_from: *y_from,
                        y_to: *y_to,
                        z_from: *z_from,
                        z_to: *z_to,
                    })
                }
            }
        }

        ret
    }

    fn volume(&self) -> u128 {
        let x = (self.x_to - self.x_from) as u128;
        let y = (self.y_to - self.y_from) as u128;
        let z = (self.z_to - self.z_from) as u128;
        x * y * z
    }

    fn is_limited(&self) -> bool {
        -50 <= self.x_from
            && self.x_from <= 50
            && -49 <= self.x_to
            && self.x_to <= 51
            && -50 <= self.y_from
            && self.y_from <= 50
            && -49 <= self.y_to
            && self.y_to <= 51
            && -50 <= self.z_from
            && self.z_from <= 50
            && -49 <= self.z_to
            && self.z_to <= 51
    }
}

fn load_reboot_steps(s: String) -> VecDeque<RebootStep> {
    let mut ret = VecDeque::new();
    let re =
        Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();

    let mut caps;
    for ln in s.lines() {
        caps = re.captures(&ln).unwrap();
        ret.push_back(RebootStep {
            on: caps.get(1).unwrap().as_str() == "on",
            cuboid: Cuboid {
                x_from: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                x_to: caps.get(3).unwrap().as_str().parse::<i32>().unwrap() + 1,
                y_from: caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                y_to: caps.get(5).unwrap().as_str().parse::<i32>().unwrap() + 1,
                z_from: caps.get(6).unwrap().as_str().parse::<i32>().unwrap(),
                z_to: caps.get(7).unwrap().as_str().parse::<i32>().unwrap() + 1,
            },
        });
    }

    ret
}

fn general_solution(mut steps: VecDeque<RebootStep>) -> u128 {
    let mut space: Vec<Cuboid> = Vec::new();
    let mut intersecting_cuboid_idx: Option<usize>;

    while let Some(reboot_step) = steps.pop_front() {
        intersecting_cuboid_idx = None;

        for cuboid_idx in 0..space.len() {
            if let Some(_) = reboot_step.cuboid.intersection(&space[cuboid_idx]) {
                intersecting_cuboid_idx = Some(cuboid_idx);
                break;
            }
        }

        if let Some(cuboid_idx) = intersecting_cuboid_idx {
            let cuboid: Cuboid = space[cuboid_idx].clone();
            let subpiece: Cuboid = reboot_step.cuboid.intersection(&cuboid).unwrap();

            // Remove the original cuboid from the space of non-intersecting cuboids.
            space.remove(cuboid_idx);
            // Insert pieces from the dissasembled original cuboids. All except
            // the intersecting one.
            for new_cuboid in cuboid
                .dissasemble(&subpiece)
                .into_iter()
                .filter(|c| *c != subpiece)
            {
                space.push(new_cuboid);
            }
            // Insert the intersecting piece only if in this step, we are adding and
            // not removing.
            if reboot_step.on {
                space.push(subpiece.clone());
            }

            // Insert new, more granular reboot steps from dissasembled reboot step cuboid.
            // The intersecting piece is not replicated here.
            for cuboid in reboot_step
                .cuboid
                .dissasemble(&subpiece)
                .into_iter()
                .filter(|c| *c != subpiece)
            {
                steps.push_front(RebootStep {
                    cuboid,
                    on: reboot_step.on,
                })
            }
        } else {
            if reboot_step.on {
                space.push(reboot_step.cuboid);
            }
        }
    }

    space.into_iter().map(|c| c.volume()).sum()
}

pub fn ex1() -> String {
    let input_str: String = fs::read_to_string("inputs/day22.txt").expect("Could not read file");
    let steps = load_reboot_steps(input_str)
        .into_iter()
        .filter(|step| step.cuboid.is_limited())
        .collect();

    general_solution(steps).to_string()
}

pub fn ex2() -> String {
    let input_str: String = fs::read_to_string("inputs/day22.txt").expect("Could not read file");
    let steps = load_reboot_steps(input_str);

    general_solution(steps).to_string()
}

#[cfg(test)]
mod tests {
    use super::{ex1, ex2};
    #[test]
    fn test_both_exercises() {
        assert_eq!(ex1(), "615869");
        assert_eq!(ex2(), "1323862415207825");
    }
}
