fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let current = parse_int_vec(&buf);
    read_line(&mut buf);

    let mut end = parse_int_vec(&buf);

    if end <= current {
        end[0] += 24;
    }

    let delta: Vec<_> = end.iter().zip(current).map(|(e, c)| e - c).collect();

    if let [mut left_hour, mut left_minute, mut left_second] = delta[..] {
        if left_second < 0 {
            left_second += 60;
            left_minute -= 1;
        }

        if left_minute < 0 {
            left_minute += 60;
            left_hour -= 1;
        }

        println!("{left_hour:02}:{left_minute:02}:{left_second:02}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.trim().split(':').map(|s| s.parse().unwrap()).collect()
}
