use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<usize>);

    const M: i64 = 1_000_000_007;
    let mut memo = [0; 3334];
    (memo[0], memo[1], memo[2]) = (1, 3, 13);

    for i in 3..=3333 {
        memo[i] = ((5 * memo[i - 1]) % M - (3 * memo[i - 2]) % M + memo[i - 3] + M) % M;
    }
    // println!("{memo:?}");
    for n in input.skip(1) {
        if n % 3 != 0 {
            println!("0");
            continue;
        }

        println!("{}", memo[n / 3]);
    }
}
