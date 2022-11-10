fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();
    let mut min_time = i32::MAX;

    for _ in 0..n {
        read_line(&mut buf);

        if let [a, b] = parse_int_vec(&buf)[..] {
            if a > b {
                continue;
            }

            let time = a.max(b);
            min_time = time.min(min_time);
        }
    }

    println!("{}", if min_time == i32::MAX { -1 } else { min_time });
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
