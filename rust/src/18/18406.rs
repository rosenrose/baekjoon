fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let score = buf.trim();
    let (left, right) = score.split_at(score.len() / 2);

    let digit_sum = |s: &str| s.chars().map(|c| c as i32 - '0' as i32).sum::<i32>();

    println!(
        "{}",
        if digit_sum(left) == digit_sum(right) {
            "LUCKY"
        } else {
            "READY"
        }
    );
}
