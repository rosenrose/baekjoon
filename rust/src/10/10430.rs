fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let nums: Vec<i32> = buf.split_whitespace().map(|s| s.parse().unwrap()).collect();

    if let [a, b, c] = nums[..] {
        println!(
            "{}\n{}\n{}\n{}",
            (a + b) % c,
            ((a % c) + (b % c)) % c,
            (a * b) % c,
            ((a % c) * (b % c)) % c
        );
    }
}
