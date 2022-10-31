fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let count = buf.trim().split(',').count();

    println!("{count}");
}
