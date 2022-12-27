use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let mut left = input().to_string();
    let mut right = String::new();
    let n: i32 = input().parse().unwrap();

    for _ in 0..n {
        match input() {
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
                if let Some(c) = input().chars().nth(0) {
                    left.push(c);
                }
            }
            _ => (),
        };
    }

    right = right.chars().rev().collect();

    println!("{left}{right}");
}
