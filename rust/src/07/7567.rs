fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let bowls = buf.trim().as_bytes();
    let height = 10
        + bowls
            .windows(2)
            .map(|window| if window[0] == window[1] { 5 } else { 10 })
            .sum::<i32>();

    println!("{height}");
}
