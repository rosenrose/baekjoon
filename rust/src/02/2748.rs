fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i64 = buf.trim().parse().unwrap();

    println!("{}", fibo(n));
}

fn fibo(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }

    let (mut a, mut b) = (1, 1);

    for _ in 2..n {
        (a, b) = (b, a + b);
    }

    b
}
