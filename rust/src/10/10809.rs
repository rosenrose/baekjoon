fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    for ch in 'a'..='z' {
        if let Some(i) = buf.find(ch) {
            print!("{i} ");
        } else {
            print!("-1 ");
        }
    }
}
