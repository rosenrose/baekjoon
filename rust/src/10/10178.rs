use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    for (c, v) in (0..input()).map(|_| (input(), input())) {
        println!(
            "You get {} piece(s) and your dad gets {} piece(s).",
            c / v,
            c % v
        );
    }
}
