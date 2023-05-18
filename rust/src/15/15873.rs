fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let input = buf.trim().as_bytes();
    let sum = match input.iter().map(|ch| ch - b'0').collect::<Vec<_>>()[..] {
        [a, b] => a + b,
        [1, 0, c] => 10 + c,
        [a, 1, 0] => 10 + a,
        _ => 20,
    };

    println!("{sum}");
}
