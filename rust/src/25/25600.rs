fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();
    let mut max_score = 0;

    for _ in 0..n {
        read_line(&mut buf);

        if let [a, d, g] = parse_int_vec(&buf)[..] {
            let mut score = a * (d + g);

            if a == d + g {
                score *= 2;
            }

            max_score = score.max(max_score);
        }
    }

    println!("{max_score}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
