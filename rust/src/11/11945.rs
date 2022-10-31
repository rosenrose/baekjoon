fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    if let [n, _] = parse_int_vec(&buf)[..] {
        for _ in 0..n {
            read_line(&mut buf);

            for c in buf.trim().chars().rev() {
                print!("{c}");
            }
            println!("");
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
