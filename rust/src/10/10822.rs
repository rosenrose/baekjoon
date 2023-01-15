fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let sum: i32 = buf.trim().split(',').flat_map(str::parse::<i32>).sum();

    println!("{sum}");
}
