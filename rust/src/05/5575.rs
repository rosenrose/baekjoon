fn main() {
    let mut buf = String::new();

    for _ in 0..3 {
        read_line(&mut buf);

        if let [on_hour, on_minute, on_second, off_hour, off_minute, off_second] =
            parse_int_vec(&buf)[..]
        {
            let mut hours = off_hour - on_hour;
            let mut minutes = off_minute - on_minute;
            let mut seconds = off_second - on_second;

            if seconds < 0 {
                seconds += 60;
                minutes -= 1;
            }

            if minutes < 0 {
                minutes += 60;
                hours -= 1;
            }

            println!("{hours} {minutes} {seconds}");
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
