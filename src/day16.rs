use std::collections::VecDeque;
use std::fs;

type Bit = u8;
type BitStream = VecDeque<Bit>;

#[derive(Debug)]
enum OpType {
    Sum,
    Product,
    Mininum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug)]
enum Packet {
    Literal {
        version: u64,
        value: u64,
    },
    Op {
        version: u64,
        op_type: OpType,
        args: Vec<Packet>,
    },
}

fn create_bit_stream(s: String) -> BitStream {
    let iter = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase().to_digit(16).unwrap() as u8);
    let mut stream: BitStream = BitStream::new();

    for v in iter {
        for k in 0..4 {
            stream.push_back((v >> (3 - k)) % 2);
        }
    }

    stream
}

fn get(s: &mut BitStream, k: usize) -> u64 {
    let mut v: u64 = 0;

    for _ in 0..k {
        v = 2 * v + s.pop_front().unwrap() as u64;
    }

    v
}

fn get_op_type(id: u64) -> OpType {
    match id {
        0 => OpType::Sum,
        1 => OpType::Product,
        2 => OpType::Mininum,
        3 => OpType::Maximum,
        5 => OpType::GreaterThan,
        6 => OpType::LessThan,
        7 => OpType::EqualTo,
        _ => panic!("Unknown op type"),
    }
}

fn get_literal_value(s: &mut BitStream) -> u64 {
    let mut value_stream: BitStream = BitStream::new();

    while let Some(1) = s.pop_front() {
        for _ in 0..4 {
            value_stream.push_back(s.pop_front().unwrap());
        }
    }

    for _ in 0..4 {
        value_stream.push_back(s.pop_front().unwrap());
    }

    let stream_len = value_stream.len();
    get(&mut value_stream, stream_len)
}

fn get_subpackets(s: &mut BitStream) -> Vec<Packet> {
    if get(s, 1) == 0 {
        let bit_length = get(s, 15);
        let mut args: Vec<Packet> = Vec::new();

        let mut temp_s: BitStream = BitStream::new();
        for _ in 0..bit_length {
            temp_s.push_back(s.pop_front().unwrap());
        }

        while temp_s.len() != 0 {
            args.push(get_packet(&mut temp_s));
        }

        return args;
    } else {
        let packet_count = get(s, 11);
        return (0..packet_count).map(|_| get_packet(s)).collect();
    }
}

fn get_packet(s: &mut BitStream) -> Packet {
    let version = get(s, 3);
    let type_id = get(s, 3);

    if type_id == 4 {
        Packet::Literal {
            version: version,
            value: get_literal_value(s),
        }
    } else {
        Packet::Op {
            version: version,
            op_type: get_op_type(type_id),
            args: get_subpackets(s),
        }
    }
}

fn sum_version_numbers(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal { version, value: _ } => *version,
        Packet::Op {
            version,
            op_type: _,
            args,
        } => {
            *version
                + args
                    .iter()
                    .map(|p| sum_version_numbers(p))
                    .reduce(|acc, k| acc + k)
                    .unwrap()
        }
    }
}

fn eval(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal { version: _, value } => *value,
        Packet::Op {
            version: _,
            op_type,
            args,
        } => match op_type {
            OpType::Sum => args.iter().map(|p| eval(p)).sum(),
            OpType::Product => args.iter().map(|p| eval(p)).product(),
            OpType::Mininum => args.iter().map(|p| eval(p)).min().unwrap(),
            OpType::Maximum => args.iter().map(|p| eval(p)).max().unwrap(),
            OpType::GreaterThan => {
                if eval(&args[0]) > eval(&args[1]) {
                    1
                } else {
                    0
                }
            }
            OpType::LessThan => {
                if eval(&args[0]) < eval(&args[1]) {
                    1
                } else {
                    0
                }
            }
            OpType::EqualTo => {
                if eval(&args[0]) == eval(&args[1]) {
                    1
                } else {
                    0
                }
            }
        },
    }
}

pub fn ex1() -> String {
    let input_str: String = fs::read_to_string("inputs/day16.txt").expect("Could not read file");
    let mut stream = create_bit_stream(input_str);

    sum_version_numbers(&get_packet(&mut stream)).to_string()
}

pub fn ex2() -> String {
    let input_str: String = fs::read_to_string("inputs/day16.txt").expect("Could not read file");
    let mut stream = create_bit_stream(input_str);

    eval(&get_packet(&mut stream)).to_string()
}

#[cfg(test)]
mod tests {
    use super::{ex1, ex2};
    #[test]
    fn test_both_exercises() {
        assert_eq!(ex1(), "920");
        assert_eq!(ex2(), "10185143721112");
    }
}
