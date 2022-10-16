fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let nums: Vec<f64> = buf.split_whitespace().map(|s| s.parse().unwrap()).collect();

    println!("{:.10}", nums[0] / nums[1]);
}
