use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Segment(u16, u16, u16, u16);

impl Segment {
    fn is_vertical(&self) -> bool {
        self.0 == self.2
    }

    fn is_horizontal(&self) -> bool {
        self.1 == self.3
    }

    fn is_diagonal(&self) -> bool {
        let deg_45 = (self.0 as i16 - self.2 as i16).abs() == (self.1 as i16 - self.3 as i16).abs();
        !self.is_horizontal() & !self.is_vertical() & deg_45
    }
}

fn get_segment(line: &str) -> Segment {
    const ERR_MSG: &str = "Incorrect line format";
    if let [x1, y1, _arrow, x2, y2] =
        line.split(&[',', ' '][..]).take(5).collect::<Vec<&str>>()[..5]
    {
        let s = Segment(
            x1.parse::<u16>().expect(ERR_MSG),
            y1.parse::<u16>().expect(ERR_MSG),
            x2.parse::<u16>().expect(ERR_MSG),
            y2.parse::<u16>().expect(ERR_MSG),
        );
        return s;
    }
    panic!("{}", ERR_MSG);
}

fn general_solution(count_diagonals: bool) -> String {
    let mut map: HashMap<(u16, u16), u16> = HashMap::new();

    let input_str = fs::read_to_string("inputs/day5.txt").expect("Could not read file");
    let segments = input_str.lines().map(|ln| get_segment(ln));

    for segment in segments {
        if segment.is_vertical() {
            let (x1, y1, _, y2) = if segment.1 < segment.3 {
                (segment.0, segment.1, segment.2, segment.3)
            } else {
                (segment.2, segment.3, segment.0, segment.1)
            };

            for i in y1..(y2 + 1) {
                let key = (x1, i);
                *map.entry(key).or_insert(0) += 1;
            }
        } else if segment.is_horizontal() {
            let (x1, y1, x2, _) = if segment.0 < segment.2 {
                (segment.0, segment.1, segment.2, segment.3)
            } else {
                (segment.2, segment.3, segment.0, segment.1)
            };

            for i in x1..(x2 + 1) {
                let key = (i, y1);
                *map.entry(key).or_insert(0) += 1;
            }
        } else if segment.is_diagonal() & count_diagonals {
            let (x1, y1, x2, y2) = if segment.0 < segment.2 {
                (segment.0, segment.1, segment.2, segment.3)
            } else {
                (segment.2, segment.3, segment.0, segment.1)
            };

            let sgn: i16 = if y1 < y2 { 1 } else { -1 };
            for i in 0..(x2 - x1 + 1) {
                let key = (x1 + i, (y1 as i16 + sgn * i as i16) as u16);
                *map.entry(key).or_insert(0) += 1;
            }
        }
    }

    map.iter().filter(|(_, v)| **v >= 2).count().to_string()
}

pub fn ex1() -> String {
    general_solution(false)
}

pub fn ex2() -> String {
    general_solution(true)
}
