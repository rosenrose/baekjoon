fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(&buf);
    let mut bars = parse_int_vec_lines(&mut buf, n);

    let (mut max, mut count) = (bars.pop().unwrap(), 1);

    while !bars.is_empty() {
        let height = bars.pop().unwrap();

        if height > max {
            max = height;
            count += 1;
        }
    }

    println!("{count}");
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
