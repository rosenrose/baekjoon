use std::f64::consts::PI;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let r: f64 = buf.trim().parse().unwrap();

    println!("{:.6}\n{:.6}", r * r * PI, r * r * 2.0);
}
