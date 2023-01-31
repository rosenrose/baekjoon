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
        [a, b, c] => 10 + if b == 0 { c } else { a },
        [_, _, _, _] => 20,
        _ => Default::default(),
    };

    println!("{sum}");
}
