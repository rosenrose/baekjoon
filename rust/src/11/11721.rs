fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let chars: Vec<String> = buf.trim().chars().map(|c| c.to_string()).collect();

    for word in chars.chunks(10) {
        println!("{}", word.concat());
    }
}
