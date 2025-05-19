use std::io;

const MAX: usize = 100_000 + 1;
const M: i32 = 9901;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut memo = [0; MAX];
    (memo[0], memo[1]) = (1, 3);

    for i in 2..=n {
        // 2 * memo[i - 1] + memo[i - 2];
        memo[i] = ((2 * memo[i - 1]) % M + memo[i - 2] % M) % M;
    }

    println!("{}", memo[n]);
}
