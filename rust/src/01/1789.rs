fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i64 = buf.trim().parse().unwrap();

    let (mut sum, mut step) = (0, 0);

    while sum <= n {
        step += 1;
        sum += step;
    }

    println!("{}", step - 1);
}
