use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines().map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let (start, end, ice, melting, water) = (input(), input(), input(), input(), input());
    let mut time = (end - start.max(0)) * water;

    if start < 0 {
        time += -start * ice;
        time += melting;
    }

    println!("{time}");
}
