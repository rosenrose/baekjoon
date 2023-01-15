use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut min = 100;
    let sum = input.filter(|num| num % 2 == 1).fold(0, |sum, num| {
        min = num.min(min);

        sum + num
    });

    if sum == 0 {
        println!("-1");
        return;
    }

    println!("{sum}\n{min}");
}
