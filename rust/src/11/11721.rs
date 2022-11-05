fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let words: Vec<char> = buf.trim().chars().collect();

    for word in words.chunks(10) {
        println!("{}", String::from_iter(word));
    }
}
