fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(&buf);

    for _ in 0..n {
        read_line(&mut buf);

        let mut s = parse_int(&buf);
        read_line(&mut buf);

        let options = parse_int(&buf);

        s += (0..options)
            .map(|_| {
                read_line(&mut buf);
                parse_int_vec(&buf).iter().product::<i32>()
            })
            .sum::<i32>();

        println!("{s}");
    }
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
