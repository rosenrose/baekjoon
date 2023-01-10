fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let sum: i32 = buf
        .trim()
        .chars()
        .map(|c| (c as i32 - '0' as i32).pow(5))
        .sum();

    println!("{sum}");
}
