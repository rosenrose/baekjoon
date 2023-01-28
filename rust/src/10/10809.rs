fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    for ch in 'a'..='z' {
        let idx = if let Some(i) = buf.find(ch) {
            i as i32
        } else {
            -1
        };

        print!("{idx} ");
    }
}
