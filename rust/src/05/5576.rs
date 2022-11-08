fn main() {
    let mut buf = String::new();

    let mut w_scores = parse_int_vec_lines(&mut buf, 10);
    w_scores.sort();
    let w_sum: i32 = w_scores[w_scores.len() - 3..].iter().sum();

    let mut k_scores = parse_int_vec_lines(&mut buf, 10);
    k_scores.sort();
    let k_sum: i32 = k_scores[k_scores.len() - 3..].iter().sum();

    println!("{w_sum} {k_sum}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}

fn parse_int_vec_lines(buf: &mut String, n: i32) -> Vec<i32> {
    (0..n)
        .map(|_| {
            read_line(buf);
            parse_int(buf)
        })
        .collect()
}
