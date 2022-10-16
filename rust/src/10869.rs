fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let nums: Vec<i32> = buf.split_whitespace().map(|s| s.parse().unwrap()).collect();

    if let [a, b] = nums[..] {
        println!("{}", a + b);
        println!("{}", a - b);
        println!("{}", a * b);
        println!("{}", a / b);
        println!("{}", a % b);
    }
}
