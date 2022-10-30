fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let sum: i32 = buf
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .sum();

    println!("{sum}");
}
