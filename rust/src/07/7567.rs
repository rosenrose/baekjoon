fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let bowls = buf.trim();

    let height = 10
        + bowls
            .chars()
            .enumerate()
            .skip(1)
            .map(|(i, c)| {
                if c == bowls.chars().nth(i - 1).unwrap() {
                    5
                } else {
                    10
                }
            })
            .sum::<i32>();

    println!("{height}");
}
