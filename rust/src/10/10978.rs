use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<usize>);

    let mut memo = [0; 21];
    memo[2] = 1;

    for i in 3..=20 {
        memo[i] = (i - 1) * (memo[i - 1] + memo[i - 2]);
    }

    for n in input.skip(1) {
        println!("{}", memo[n]);
    }
}
