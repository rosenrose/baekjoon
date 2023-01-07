use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
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
