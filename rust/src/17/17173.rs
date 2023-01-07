use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    let n = input.next().unwrap();
    let mut sum = 0;
    let mut sieve = vec![false; n + 1];

    for num in input.skip(1) {
        for i in (num..=n).step_by(num) {
            if sieve[i] {
                continue;
            }

            sum += i;
            sieve[i] = true;
        }
    }

    println!("{sum}");
}
