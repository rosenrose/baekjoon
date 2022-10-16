fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let nums: Vec<i32> = buf.split_whitespace().map(|s| s.parse().unwrap()).collect();

    if let [a, b, c] = nums[..] {
        println!("{}", (a + b) % c);
        println!("{}", ((a % c) + (b % c)) % c);
        println!("{}", (a * b) % c);
        println!("{}", ((a % c) * (b % c)) % c);
    }
}
