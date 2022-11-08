fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(buf.trim());

    for _ in 0..n {
        read_line(&mut buf);

        if let [first, second] = parse_str_vec(&buf)[..] {
            let first_value: i32 = first
                .chars()
                .rev()
                .enumerate()
                .map(|(i, c)| (c as u8 - 'A' as u8) as i32 * 26_i32.pow(i as u32))
                .sum();

            let second_value = parse_int(second);

            println!(
                "{}",
                if first_value.abs_diff(second_value) <= 100 {
                    "nice"
                } else {
                    "not nice"
                }
            );
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}

fn parse_str_vec(buf: &String) -> Vec<&str> {
    buf.trim().split('-').collect()
}
