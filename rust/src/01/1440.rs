fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b, c] = parse_int_vec(buf.trim())[..] else { return };
    let mut count = 0;

    if matches!((a, b, c), (1..=12, 0..=59, 0..=59)) {
        count += 2;
    }
    if matches!((a, b, c), (0..=59, 1..=12, 0..=59)) {
        count += 2;
    }
    if matches!((a, b, c), (0..=59, 0..=59, 1..=12)) {
        count += 2;
    }

    println!("{count}");
}

fn parse_int_vec(s: &str) -> Vec<i32> {
    s.split(':').flat_map(str::parse).collect()
}
