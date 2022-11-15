fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [sum, diff] = parse_int_vec(&buf)[..] {
        let (a, b) = ((sum + diff) / 2, (sum - diff) / 2);

        if (a + b != sum) || (a - b != diff) || a < 0 || b < 0 || a < b {
            println!("-1");
            return;
        }

        println!("{a} {b}");
    }
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}