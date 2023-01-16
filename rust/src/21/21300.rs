fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    println!(
        "{}",
        buf.split_whitespace()
            .flat_map(str::parse::<i32>)
            .sum::<i32>()
            * 5
    );
}
