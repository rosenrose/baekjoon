fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    if let [mut hour, mut minute] = parse_int_vec(&buf)[..] {
        read_line(&mut buf);
        let cooking_time = parse_int(&buf);

        minute += cooking_time;

        if minute >= 60 {
            hour = (hour + (minute / 60)) % 24;
            minute %= 60;
        }

        println!("{hour} {minute}");
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
