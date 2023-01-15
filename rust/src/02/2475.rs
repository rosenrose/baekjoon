fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let nums = buf.split_whitespace().flat_map(str::parse::<i32>);

    let verify = nums.map(|n| n * n).sum::<i32>() % 10;

    println!("{verify}");
}
