use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

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
