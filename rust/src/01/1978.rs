use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    println!("{}", input.skip(1).filter(|&num| is_prime(num)).count());
}

fn is_prime(num: i32) -> bool {
    if num == 1 {
        return false;
    }

    (2..).take_while(|i| i * i <= num).all(|i| num % i != 0)
}
