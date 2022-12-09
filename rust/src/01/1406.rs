use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let mut left = input.next().unwrap().to_string();
    let mut right = String::new();
    let n: i32 = input.next().unwrap().parse().unwrap();

    for _ in 0..n {
        match input.next().unwrap() {
            "L" => {
                if let Some(c) = left.pop() {
                    right.push(c);
                }
            }
            "D" => {
                if let Some(c) = right.pop() {
                    left.push(c);
                }
            }
            "B" => {
                left.pop();
            }
            "P" => {
                if let Some(c) = input.next().unwrap().chars().nth(0) {
                    left.push(c);
                }
            }
            _ => (),
        };
    }

    right = right.chars().rev().collect();

    println!("{left}{right}");
}
