use std::f64::consts::PI;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let r: i32 = buf.trim().parse().unwrap();

    println!(
        "{:.6}\n{:.6}",
        (r.pow(2) as f64) * PI,
        (r.pow(2) as f64) * 2.0
    );
}
