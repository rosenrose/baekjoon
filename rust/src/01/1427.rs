fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut chars: Vec<_> = buf.trim().chars().collect();
    chars.sort();

    println!("{}", String::from_iter(chars.iter().rev()));
}
