use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let n = input.next().unwrap();
    let extra_apple = (0..n).fold(0, |acc, _| {
        let (student, apple) = (input.next().unwrap(), input.next().unwrap());
        acc + (apple % student)
    });

    println!("{extra_apple}");
}
