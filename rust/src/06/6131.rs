fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let mut count = 0;

    for a in (2..=500).skip_while(|a| a * a <= n) {
        let a2 = a * a;
        let b = (a2 as f64 - n as f64).sqrt() as i32;

        if a2 == b * b + n {
            count += 1;
        }
    }

    println!("{count}");
}
