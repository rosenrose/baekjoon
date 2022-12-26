fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut n: i32 = buf.trim().parse().unwrap();
    let mut step = 1;

    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        step += 1;
    }

    println!("{step}");
}
