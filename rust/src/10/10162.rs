fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut t: i32 = buf.trim().parse().unwrap();

    if t % 10 != 0 {
        println!("-1");
        return;
    }

    let (mut a, mut b, mut c) = (0, 0, 0);

    a += t / 300;
    t %= 300;

    b += t / 60;
    t %= 60;

    c += t / 10;

    println!("{a} {b} {c}");
}
