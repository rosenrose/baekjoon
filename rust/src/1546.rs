fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let score = buf.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let max = score.clone().max().unwrap();

    let new_score = score.clone().map(|s| (s as f64 / max as f64) * 100.0);
    let sum: f64 = new_score.sum();

    println!("{:.10}", sum / score.count() as f64);
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
