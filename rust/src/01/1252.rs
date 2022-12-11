fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let sum: i128 = buf
        .split_whitespace()
        .map(|s| i128::from_str_radix(s, 2).unwrap())
        .sum();

    println!("{sum:b}");
}
