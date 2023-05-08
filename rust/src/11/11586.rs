use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let (_, state) = (input.next(), input.next_back().unwrap());

    match state {
        "1" => input.for_each(|face| println!("{face}")),
        "2" => input.for_each(|face| println!("{}", face.chars().rev().collect::<String>())),
        "3" => input.rev().for_each(|face| println!("{face}")),
        _ => unreachable!(),
    }
}
