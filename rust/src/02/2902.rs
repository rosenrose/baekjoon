fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let short = buf.trim().chars().filter(|c| c.is_uppercase());

    for c in short {
        print!("{c}");
    }
}
