fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let input = buf.trim();
    let n: i64 = input.parse().unwrap();

    let min = (n - (9 * input.len() as i64)).max(1);
    let max = n - 1;

    if let Some(min_generator) = (min..=max).find(|&generator| digit_sum(generator) == n) {
        println!("{min_generator}");
    } else {
        println!("0");
    }
}

fn digit_sum(n: i64) -> i64 {
    let sum: i64 = n
        .to_string()
        .as_bytes()
        .iter()
        .map(|ch| (ch - b'0') as i64)
        .sum();

    n + sum
}
