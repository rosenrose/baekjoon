fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let counts = parse_int_vec(&buf);
    read_line(&mut buf);

    if let [b, c] = parse_int_vec(&buf)[..] {
        let watchers = counts
            .iter()
            .map(|&count| 1 + ((count - b).max(0) as f64 / c as f64).ceil() as i64);

        println!("{}", watchers.sum::<i64>());
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
