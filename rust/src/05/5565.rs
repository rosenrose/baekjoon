use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    let total_sum = input.next().unwrap();
    let rest_sum: i32 = input.sum();

    println!("{}", total_sum - rest_sum);
}
