fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for i in 1..=n {
        read_line(&mut buf);

        let words: Vec<&str> = buf.split_whitespace().rev().collect();

        println!("Case #{i}: {}", words.join(" "));
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
