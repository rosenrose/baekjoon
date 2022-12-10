use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    let total_sum = input.next().unwrap();
    let rest_sum: i32 = input.sum();

    println!("{}", total_sum - rest_sum);
}
