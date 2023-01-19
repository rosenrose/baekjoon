use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let _n = input.next();
    let state: i32 = input.next_back().unwrap().parse().unwrap();

    match state {
        1 => input.for_each(|face| println!("{face}")),
        2 => input.for_each(|face| println!("{}", face.chars().rev().collect::<String>())),
        3 => input.rev().for_each(|face| println!("{face}")),
        _ => (),
    }
}
