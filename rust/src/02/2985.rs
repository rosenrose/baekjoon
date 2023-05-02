fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    match parse_int_vec(&buf)[..] {
        [a, b, c] if a + b == c => println!("{a}+{b}={c}"),
        [a, b, c] if a - b == c => println!("{a}-{b}={c}"),
        [a, b, c] if a * b == c => println!("{a}*{b}={c}"),
        [a, b, c] if a / b == c && a % b == 0 => println!("{a}/{b}={c}"),
        [a, b, c] if a == b + c => println!("{a}={b}+{c}"),
        [a, b, c] if a == b - c => println!("{a}={b}-{c}"),
        [a, b, c] if a == b * c => println!("{a}={b}*{c}"),
        [a, b, c] if a == b / c && b % c == 0 => println!("{a}={b}/{c}"),
        _ => (),
    }
}

fn parse_int_vec(buf: &str) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
