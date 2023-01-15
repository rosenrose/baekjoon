use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();
    let [a, b] = parse_int_vec(&buf)[..] else { return };
    let (a, b) = (a.min(b), a.max(b));

    println!("{}", (b - a - 1).max(0));

    for num in a + 1..b {
        write!(output, "{num} ").unwrap();
    }

    print!("{output}");
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
