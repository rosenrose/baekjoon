fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let points = parse_int_vec(&buf);
    read_line(&mut buf);

    if let [x, _, _] = parse_int_vec(&buf)[..] {
        let number = match points.iter().position(|&p| p == x) {
            Some(i) => i + 1,
            None => 0,
        };

        println!("{number}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
