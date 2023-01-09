fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let (a, b) = (input.next().unwrap(), input.next().unwrap());

    let digit_sum = |s: &str| -> i64 { s.chars().map(|c| c as i64 - '0' as i64).sum() };

    println!("{}", digit_sum(a) * digit_sum(b));
}
