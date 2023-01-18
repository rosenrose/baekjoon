fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    const M: i32 = 9901;

    let mut cache = vec![0; n + 1];
    (cache[0], cache[1]) = (1, 3);

    for i in 2..=n {
        // 2 * cache[i - 1] + cache[i - 2];
        cache[i] = ((2 * cache[i - 1]) % M + cache[i - 2] % M) % M;
    }

    println!("{}", cache[n]);
}
