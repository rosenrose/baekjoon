fn main() {
    let mut buf = String::new();

    for _ in 0..3 {
        read_line(&mut buf);

        let fronts = buf.split_whitespace().filter(|&s| s == "0").count();

        let yut = match fronts {
            0 => 'E',
            1..=4 => ('A' as u8 + fronts as u8 - 1) as char,
            _ => '\0',
        };

        println!("{yut}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
