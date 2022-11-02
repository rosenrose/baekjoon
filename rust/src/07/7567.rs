fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let bowls = buf.trim().as_bytes();

    let height = 10
        + bowls
            .iter()
            .enumerate()
            .skip(1)
            .map(|(i, &c)| if c == bowls[i - 1] { 5 } else { 10 })
            .sum::<i32>();

    println!("{height}");
}
