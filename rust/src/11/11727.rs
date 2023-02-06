fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    const M: i32 = 10_007;

    match n {
        1 => println!("1"),
        2 => println!("3"),
        _ => {
            let (mut a, mut b) = (1, 3);

            for _ in 0..n - 2 {
                (a, b) = (b, (a * 2 + b) % M);
            }

            println!("{b}");
        }
    }
}
