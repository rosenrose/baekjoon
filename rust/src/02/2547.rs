fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(&buf);

    for _ in 0..n {
        read_line(&mut buf);
        read_line(&mut buf);

        let students = parse_int(&mut buf);
        let candies: i128 = (0..students)
            .map(|_| {
                read_line(&mut buf);
                parse_int(&buf)
            })
            .sum();

        println!("{}", if candies % students == 0 { "YES" } else { "NO" });
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i128 {
    buf.trim().parse().unwrap()
}
