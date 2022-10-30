fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);

        if buf.trim().is_empty() {
            break;
        }

        let sum: i32 = buf
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .sum();

        println!("{sum}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
