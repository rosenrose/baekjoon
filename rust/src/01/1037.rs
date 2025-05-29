use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (min, max) = input.skip(1).fold((i32::MAX, i32::MIN), |(min, max), num| {
        (num.min(min), num.max(max))
    });

    println!("{}", min * max);
}
