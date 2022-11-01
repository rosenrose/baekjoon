fn main() {
    let mut buf = String::new();

    if let [a, b, c, d, p] = parse_int_vec_lines(&mut buf, 5)[..] {
        let x_bill = a * p;
        let mut y_bill = b;

        if p > c {
            y_bill += (p - c) * d;
        }

        println!("{}", x_bill.min(y_bill));
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}

fn parse_int_vec_lines(buf: &mut String, n: i32) -> Vec<i32> {
    (0..n)
        .map(|_| {
            read_line(buf);
            parse_int(buf)
        })
        .collect()
}
