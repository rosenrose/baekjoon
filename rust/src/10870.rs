fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    println!("{}", fibo(n));
}

fn fibo(n: i32) -> i32 {
    if n <= 1 {
        n
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}
