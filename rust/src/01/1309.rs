fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    const M: i32 = 9901;

    let mut memo = vec![0; n + 1];
    (memo[0], memo[1]) = (1, 3);

    for i in 2..=n {
        // 2 * memo[i - 1] + memo[i - 2];
        memo[i] = ((2 * memo[i - 1]) % M + memo[i - 2] % M) % M;
    }

    println!("{}", memo[n]);
}
