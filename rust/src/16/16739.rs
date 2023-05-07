#[derive(Eq, PartialEq, Debug)]
enum Items {
    Number(i32),
    Letter(char),
}

use std::cmp::Ordering;
use std::fmt::Write;
use std::io;
use Items::*;

impl Ord for Items {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Number(a), Number(b)) => a.cmp(b),
            (Number(_), Letter(_)) => Ordering::Less,
            (Letter(_), Number(_)) => Ordering::Greater,
            (Letter(a), Letter(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Items {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().skip(1).map(parse_to_sequence);
    let mut output = String::new();

    let s0 = input.next().unwrap();

    for sn in input {
        writeln!(
            output,
            "{}",
            match sn.cmp(&s0) {
                Ordering::Less => "-",
                _ => "+",
            }
        )
        .unwrap();
    }

    print!("{output}");
}

fn parse_to_sequence(input: &str) -> Vec<Items> {
    let mut sequence = Vec::new();
    let mut num = String::new();

    for ch in input.chars() {
        match ch {
            '0'..='9' => num.push(ch),
            _ => {
                if !num.is_empty() {
                    sequence.push(Number(num.parse().unwrap()));
                    num.clear();
                }

                sequence.push(Letter(ch));
            }
        }
    }

    if !num.is_empty() {
        sequence.push(Number(num.parse().unwrap()));
    }

    sequence
}
