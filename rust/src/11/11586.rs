use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines();
    input.next();

    let state: i32 = input.next_back().unwrap().parse().unwrap();

    match state {
        1 => input.for_each(|face| println!("{face}")),
        2 => input.for_each(|face| println!("{}", face.chars().rev().collect::<String>())),
        3 => input.rev().for_each(|face| println!("{face}")),
        _ => (),
    }
}
