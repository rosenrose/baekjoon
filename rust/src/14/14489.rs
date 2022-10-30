fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    if let [a, b] = parse_int_vec(&buf)[..] {
        read_line(&mut buf);
        let c: i32 = buf.trim().parse().unwrap();

        let balance = a + b;
        let price = c * 2;

        println!(
            "{}",
            if balance < price {
                balance
            } else {
                balance - price
            }
        );
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
