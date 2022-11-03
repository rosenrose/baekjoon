fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(&buf);

    loop {
        read_line(&mut buf);
        let num = parse_int(&buf);

        if num == 0 {
            return;
        }

        println!(
            "{num} is {}a multiple of {n}.",
            if num % n == 0 { "" } else { "NOT " }
        );
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
