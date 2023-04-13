use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let mut left = input.next().unwrap().to_owned();
    let mut right = String::new();

    for op in input.skip(1) {
        match op {
            "L" => {
                if let Some(ch) = left.pop() {
                    right.push(ch);
                }
            }
            "D" => {
                if let Some(ch) = right.pop() {
                    left.push(ch);
                }
            }
            "B" => {
                left.pop();
            }
            _ => {
                let (_, param) = op.split_once(' ').unwrap();
                left.push_str(param);
            }
        };
    }

    right = right.chars().rev().collect();

    println!("{left}{right}");
}
