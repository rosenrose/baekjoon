fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let k: i32 = buf.trim().parse().unwrap();
    let (mut a, mut b) = (1, 0);

    for _ in 0..k {
        (a, b) = (b, a + b);
    }

    println!("{a} {b}");
}
