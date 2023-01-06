fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    if n < 2 {
        println!("1");
        return;
    }

    const M: i32 = 1_000_000_007;
    let (mut a, mut b) = (1, 1);

    for _ in 0..n - 1 {
        (a, b) = (b, (a % M + b % M + 1) % M);
    }

    println!("{b}");
}
