fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    println!("{}", fibo_rem(n));
}

fn fibo_rem(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    let (mut a, mut b) = (1, 1);
    const M: i32 = 1_000_000_007;

    for _ in 2..n {
        (a, b) = (b, (a % M + b % M) % M);
    }

    b
}
