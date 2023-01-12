fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let ball = buf
        .trim()
        .chars()
        .fold(1, |ball, order| match (ball, order) {
            (1, 'A') | (3, 'B') => 2,
            (2, 'A') | (3, 'C') => 1,
            (1, 'C') | (2, 'B') => 3,
            _ => ball,
        });

    println!("{ball}");
}
