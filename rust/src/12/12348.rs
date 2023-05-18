fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let input = buf.trim();
    let n: i64 = input.parse().unwrap();

    let min = (n - (9 * input.len() as i64)).max(1);
    let max = n - 1;

    for i in min..=max {
        if d(i) == n {
            println!("{i}");
            return;
        }
    }

    println!("0");
}

fn d(n: i64) -> i64 {
    let digit_sum: i64 = n
        .to_string()
        .as_bytes()
        .iter()
        .map(|ch| (ch - b'0') as i64)
        .sum();
    n + digit_sum
}
