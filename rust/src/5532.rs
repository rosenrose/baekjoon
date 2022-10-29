fn main() {
    let mut buf = String::new();

    if let [l, a, b, c, d] = parse_int_vec_lines(&mut buf, 5)[..] {
        let lang_count = (a as f64 / c as f64).ceil() as i32;
        let math_count = (b as f64 / d as f64).ceil() as i32;

        println!("{}", l - lang_count.max(math_count));
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
