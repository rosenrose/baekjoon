fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let abcd = parse_int_vec(&buf);
    read_line(&mut buf);

    let pmn = parse_int_vec(&buf);

    if let [a, b, c, d, p, m, n] = [abcd, pmn].concat()[..] {
        let dog_atk_count = |time: i32, active: i32, rest: i32| {
            if time % (active + rest) < active {
                1
            } else {
                0
            }
        };

        for time in [p - 1, m - 1, n - 1] {
            println!("{}", dog_atk_count(time, a, b) + dog_atk_count(time, c, d));
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
