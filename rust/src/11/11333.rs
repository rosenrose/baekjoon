use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<usize>);

    const M: i64 = 1_000_000_007;
    let mut memo = [0_i64; 3334];
    (memo[0], memo[1], memo[2]) = (1, 3, 13);

    for i in 3..=3333 {
        memo[i] = ((5 * memo[i - 1]) - (3 * memo[i - 2]) + memo[i - 3]).rem_euclid(M);
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
