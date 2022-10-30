fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    if let [mut hour, mut minute, mut second] = parse_int_vec(&buf)[..] {
        read_line(&mut buf);
        let mut time: i32 = buf.trim().parse().unwrap();

        second += time % 60;
        minute += second / 60;
        second %= 60;

        time -= time % 60;

        minute += (time % 3600) / 60;
        hour += minute / 60;
        minute %= 60;

        time -= time % 3600;

        hour += time / 3600;
        hour %= 24;

        println!("{hour} {minute} {second}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
