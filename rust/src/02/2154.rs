fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim();
    let nums: String = (1..=n.parse::<i32>().unwrap())
        .map(|n| n.to_string())
        .collect();

    println!("{}", nums.find(n).unwrap() + 1);
}
