use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines();

    let mut lhs: i32 = input.next().unwrap().parse().unwrap();
    let mut op = "";

    for line in input {
        if let Ok(rhs) = line.parse::<i32>() {
            match op {
                "+" => lhs += rhs,
                "-" => lhs -= rhs,
                "*" => lhs *= rhs,
                "/" => lhs /= rhs,
                _ => (),
            }
        } else {
            op = line;
        }
    }

    println!("{lhs}");
}
