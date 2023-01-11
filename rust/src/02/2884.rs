fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [mut hour, mut minute] = parse_int_vec(&buf)[..] else { return };
    let offset = 45;

    if minute < offset {
        minute = 60 - (offset - minute);
        hour = (hour + (24 - 1)) % 24;
    } else {
        minute -= offset;
    }

    println!("{hour} {minute}");
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
