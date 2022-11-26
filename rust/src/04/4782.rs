fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);

        if buf.trim() == "0 0" {
            return;
        }

        if let [b, n] = parse_int_vec(&buf)[..] {
            for m in (1..=2 * n).filter(|&m| m != n).rev() {
                let dividend = b * m * (2 * n - m);
                let divisor = n * n;

                if dividend % divisor != 0 {
                    continue;
                }

                let a = dividend / divisor;
                print!("{a}/{m} ");
            }
        }

        println!("");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
