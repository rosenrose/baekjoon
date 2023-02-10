fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let sum: i32 = buf
        .split_whitespace()
        .flat_map(|s| s.chars().rev().collect::<String>().parse::<i32>())
        .sum::<i32>()
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap();

    println!("{sum}");
}
