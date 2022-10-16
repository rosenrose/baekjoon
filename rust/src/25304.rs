fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let x = parse_int(&buf);
    read_line(&mut buf);

    let n = parse_int(&buf);

    let price_count = (0..n).map(|_| {
        read_line(&mut buf);
        parse_int_vec(&buf)
    });

    let sum = price_count.fold(0, |a, b| a + (b[0] * b[1]));

    println!("{}", if sum == x { "Yes" } else { "No" });
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
