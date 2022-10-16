fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);

        let scores = buf
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse::<i32>().unwrap());
        let len = scores.clone().count();

        let sum: i32 = scores.clone().sum();
        let avg = sum as f64 / len as f64;

        let over_avg_count = scores.filter(|&s| s as f64 > avg).count();
        let over_avg_ratio = over_avg_count as f64 / len as f64;

        println!("{:.3}%", over_avg_ratio * 100.0);
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
