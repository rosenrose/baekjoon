fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let (a, b) = buf.trim().split_once(' ').unwrap();
    let digit_sum = |s: &str| s.chars().map(|c| c as i64 - '0' as i64).sum::<i64>();

    println!("{}", digit_sum(a) * digit_sum(b));
}
