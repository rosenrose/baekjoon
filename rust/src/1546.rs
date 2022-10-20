fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let scores = parse_int_vec(&buf);
    let max = *scores.iter().max().unwrap();

    let new_scores = scores.iter().map(|&s| (s as f64 / max as f64) * 100.0);
    let sum: f64 = new_scores.sum();

    println!("{:.10}", sum / scores.len() as f64);
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
