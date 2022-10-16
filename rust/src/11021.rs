fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for i in 1..=n {
        read_line(&mut buf);

        let sum: i32 = buf
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .sum();

        println!("Case #{i}: {sum}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
