fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut ball = 1;

    for order in buf.trim().chars() {
        ball = match (ball, order) {
            (1, 'A') | (3, 'B') => 2,
            (2, 'A') | (3, 'C') => 1,
            (1, 'C') | (2, 'B') => 3,
            _ => ball,
        };
    }

    println!("{ball}");
}
