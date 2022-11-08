fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(&buf);

    for _ in 0..n {
        read_line(&mut buf);
        let dec = parse_int(&buf);

        format!("{dec:b}")
            .chars()
            .rev()
            .enumerate()
            .filter(|&(_, c)| c == '1')
            .for_each(|(i, _)| print!("{i} "));
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
