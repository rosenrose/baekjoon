fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let s: i32 = parse_int_vec(&buf).iter().sum();
    read_line(&mut buf);

    let t: i32 = parse_int_vec(&buf).iter().sum();

    println!("{}", s.max(t));
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
