fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(&buf);

    for _ in 0..n {
        read_line(&mut buf);

        let m = parse_int(&buf);
        let rewards = parse_matrix(&mut buf, m);
        read_line(&mut buf);

        if let [k, d, a] = parse_int_vec(&buf)[..] {
            let donation = rewards.iter().fold(0, |acc, reward| {
                acc + (reward[0] * k - reward[1] * d + reward[2] * a).max(0)
            });

            println!("{donation}");
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i64 {
    buf.trim().parse().unwrap()
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn parse_matrix(buf: &mut String, rows: i64) -> Vec<Vec<i64>> {
    (0..rows)
        .map(|_| {
            read_line(buf);
            parse_int_vec(buf)
        })
        .collect()
}
