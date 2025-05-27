use std::io;

const MAX: usize = 1000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut sum = 0;
    let mut sieve = [false; MAX];

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
