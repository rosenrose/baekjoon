fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    if let [l, p] = parse_int_vec(&buf)[..] {
        let exact_count = l * p;
        read_line(&mut buf);

        let counts = parse_int_vec(&buf);

        for count in counts {
            print!("{} ", count - exact_count);
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
