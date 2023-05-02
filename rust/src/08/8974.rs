fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [a, b] = parse_int_vec(&buf)[..] else { return };
    let sequence = (1..).flat_map(|n| std::iter::repeat(n).take(n));

    println!("{}", sequence.skip(a - 1).take(b - a + 1).sum::<usize>());
}

fn parse_int_vec(buf: &str) -> Vec<usize> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
