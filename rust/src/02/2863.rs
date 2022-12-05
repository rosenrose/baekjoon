fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let ab = parse_int_vec(&buf);
    read_line(&mut buf);

    let cd = parse_int_vec(&buf);

    if let [a, b, c, d] = [ab, cd].concat()[..] {
        let numerator = a * d + b * c;
        let denominators = [c * d, b * d, a * b, a * c];

        let (min_rotate, _) = denominators
            .iter()
            .enumerate()
            .max_by(|(_, &a), (_, &b)| (numerator * b).cmp(&(numerator * a)))
            .unwrap();

        println!("{min_rotate}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
