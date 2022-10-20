fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let nums = buf.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let verify = nums.map(|n| n * n).sum::<i32>() % 10;

    println!("{verify}");
}
