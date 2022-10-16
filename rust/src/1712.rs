fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [a, b, c] = parse_int_vec(&buf)[..] {
        if b >= c {
            println!("{}", -1);
            return;
        }

        println!("{}", (a / (c - b)) + 1);
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
