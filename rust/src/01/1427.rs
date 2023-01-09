fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut chars: Vec<_> = buf.trim().chars().collect();
    chars.sort_by(|a, b| b.cmp(&a));

    for c in chars {
        print!("{c}");
    }
}
