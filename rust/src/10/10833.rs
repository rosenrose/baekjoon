use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let extra_apple = (0..input()).fold(0, |acc, _| {
        let (student, apple) = (input(), input());
        acc + (apple % student)
    });

    println!("{extra_apple}");
}
