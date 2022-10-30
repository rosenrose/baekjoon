fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let total_sum = parse_int(&buf);
    let rest_sum: i32 = (0..9)
        .map(|_| {
            read_line(&mut buf);
            parse_int(&buf)
        })
        .sum();

    println!("{}", total_sum - rest_sum);
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
