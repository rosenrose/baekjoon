fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let sum = match buf
        .trim()
        .chars()
        .map(|c| c as u8 - '0' as u8)
        .collect::<Vec<_>>()[..]
    {
        [a, b] => a + b,
        [1, 0, c] => 10 + c,
        [a, 1, 0] => 10 + a,
        _ => 20,
    };

    println!("{sum}");
}
