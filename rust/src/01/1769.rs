fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let x = buf.trim();
    let mut count = 0;

    let sum = if x.len() < 2 {
        x.parse().unwrap()
    } else {
        convert(x, &mut count)
    };

    println!("{count}\n{}", if sum % 3 == 0 { "YES" } else { "NO" });
}

fn convert(num: &str, count: &mut i32) -> i32 {
    *count += 1;

    let sum: i32 = num.as_bytes().iter().map(|ch| (ch - b'0') as i32).sum();

    if sum < 10 {
        sum
    } else {
        convert(&sum.to_string(), count)
    }
}
