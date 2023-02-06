fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    for count in ('a'..='z').map(|ch| buf.matches(ch).count()) {
        print!("{count} ");
    }
}
