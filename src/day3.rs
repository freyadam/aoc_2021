use std::fs;

fn find_value<'a>(mut selection: Vec<&'a str>, negate: bool) -> &'a str {
    // preprocessing
    selection.sort();
    selection.dedup();

    let vec_len = selection[0].len();

    for k in 0..vec_len {
        // compute most frequent values
        let mut size_counter: u16 = 0;
        let mut v: u16 = 0;
        for line in selection.iter() {
            size_counter += 1;
            if line.as_bytes()[k] == '1' as u8 {
                v += 1;
            }
        }

        let (x, y) = if 2 * v >= size_counter {
            ('1' as u8, '0' as u8)
        } else {
            ('0' as u8, '1' as u8)
        };

        if selection.len() == 1 {
            break;
        } else {
            selection = selection
                .iter()
                .filter(|v| v.as_bytes()[k] == (if negate { x } else { y }))
                .map(|x| *x)
                .collect();
        }
    }

    assert!(selection.len() == 1);
    selection[0]
}

pub fn ex1() -> String {
    // load lines
    let content_str = fs::read_to_string("inputs/day3.txt").expect("Could not read file");

    // allocate vector
    let vec_len = content_str.lines().next().expect("Empty file").len();
    let mut vec: Vec<u16> = Vec::with_capacity(vec_len);
    for _ in 0..vec_len {
        vec.push(0);
    }

    // compute most frequent values
    let mut size_counter: u16 = 0;
    for line in content_str.lines() {
        size_counter += 1;
        for (k, c) in line.chars().enumerate() {
            if c == '1' {
                vec[k] += 1;
            }
        }
    }

    // compute result
    let mut gamma: u16 = 0;
    let mut epsilon: u16 = 0;
    for v in vec.iter() {
        gamma += 1;
        if 2 * v >= size_counter {
        } else {
            epsilon += 1;
        }
        gamma *= 2;
        epsilon *= 2;
    }
    gamma /= 2;
    epsilon /= 2;

    // print result
    (gamma as u32 * epsilon as u32).to_string()
}

pub fn ex2() -> String {
    // load lines
    let content_str = fs::read_to_string("inputs/day3.txt").expect("Could not read file");

    let oxygen: u16 =
        u16::from_str_radix(find_value(content_str.lines().collect(), false), 2).expect("");
    let co2: u16 =
        u16::from_str_radix(find_value(content_str.lines().collect(), true), 2).expect("");

    (oxygen as u32 * co2 as u32).to_string()
}
