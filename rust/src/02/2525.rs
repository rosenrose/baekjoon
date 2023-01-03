use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let (mut hour, mut minute, cooking_time) = (input(), input(), input());

    minute += cooking_time;

    if minute >= 60 {
        hour = (hour + (minute / 60)) % 24;
        minute %= 60;
    }

    println!("{hour} {minute}");
}
