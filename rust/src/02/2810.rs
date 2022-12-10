use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let seats = buf.lines().next_back().unwrap();
    let count = seats.len();

    let mut seats: String = seats
        .split_inclusive("LL")
        .flat_map(|s| s.split_inclusive('S'))
        .map(|s| format!("*{s}"))
        .collect();

    seats.push('*');
    // println!("{seats}");

    seats = seats
        .replace("*S", "")
        .replace("*L", "")
        .replace("S*", "")
        .replace("L*", "");

    let no_holders = seats.replace('*', "").len();

    println!("{}", count - no_holders);
}
