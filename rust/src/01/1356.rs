fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n = buf.trim();
    let digit_product = |s: &str| {
        s.as_bytes()
            .iter()
            .map(|ch| (ch - b'0') as i32)
            .product::<i32>()
    };

    println!(
        "{}",
        if (1..=n.len() - 1).any(|i| {
            let (left, right) = n.split_at(i);
            digit_product(left) == digit_product(right)
        }) {
            "YES"
        } else {
            "NO"
        }
    );
}
