fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    const M: i64 = 1_000_000_007;

    match n {
        1 => println!("2"),
        2 => println!("7"),
        _ => {
            let (mut a, mut b) = (2, 7);
            let mut sum = 1 + a + b;

            for _ in 0..n - 2 {
                (a, b) = (b, (a + sum * 2) % M);
                sum += b;
            }

            println!("{b}");
        }
    }
}
