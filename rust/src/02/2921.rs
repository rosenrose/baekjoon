fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let mut sum = 0;

    for a in 0..=n {
        for b in a..=n {
            sum += a + b;
        }
    }

    println!("{sum}");
}
