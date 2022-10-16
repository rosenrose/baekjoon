fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let t: i32 = buf.trim().parse().unwrap();

    for _ in 0..t {
        read_line(&mut buf);

        if let [h, _, n] = parse_int_vec(&buf)[..] {
            let floor = if n % h == 0 { h } else { n % h };
            let num_from_elevator = (n as f64 / h as f64).ceil() as i32;

            println!("{floor}{num_from_elevator:02}");
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
