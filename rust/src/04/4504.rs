use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);
    let n = input.next().unwrap();

    for num in input.take_while(|&num| num != 0) {
        println!(
            "{num} is {} of {n}.",
            if num % n == 0 {
                "a multiple"
            } else {
                "NOT a multiple"
            }
        );
    }
}
