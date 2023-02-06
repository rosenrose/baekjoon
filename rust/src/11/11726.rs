fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    const M: i32 = 10_007;

    if n <= 2 {
        println!("{n}");
        return;
    }

    let (mut a, mut b) = (1, 2);

    for _ in 0..n - 2 {
        (a, b) = (b, (a + b) % M);
    }

    println!("{b}");
}
