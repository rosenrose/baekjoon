fn main() {
    let mut buf = String::new();

    if let [start, end, ice, melting, water] = parse_int_vec_lines(&mut buf, 5)[..] {
        let mut time = (end - start.max(0)) * water;

        if start < 0 {
            time += -start * ice;
            time += melting;
        }

        println!("{time}");
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
