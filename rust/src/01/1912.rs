use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut max = -1000;

    let _ = input.skip(1).fold(0, |acc, num| {
        let sum = num.max(acc + num);
        max = sum.max(max);

        sum
    });

    println!("{max}");
}
