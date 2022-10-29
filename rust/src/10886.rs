fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(&buf);

    let is_cute = (0..n)
        .map(|_| {
            read_line(&mut buf);
            parse_int(&buf)
        })
        .filter(|&i| i == 1)
        .count()
        > n / 2;

    println!("Junhee is {}!", if is_cute { "cute" } else { "not cute" });
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> usize {
    buf.trim().parse().unwrap()
}
