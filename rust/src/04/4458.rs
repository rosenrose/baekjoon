fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);
        let sentence = buf.trim();

        println!(
            "{}{}",
            sentence.chars().nth(0).unwrap().to_uppercase(),
            &sentence[1..]
        );
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
