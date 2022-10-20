fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);
        let input = buf.trim();

        if input.is_empty() {
            return;
        }

        println!("{input}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
