fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let k = (-1 + ((4 * n - 3) as f64).sqrt() as i32) / 2;

    println!("{k}");
}
