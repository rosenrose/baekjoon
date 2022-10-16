fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let width = n * 2 - 1;

    for i in (1..=n).rev().chain(2..=n) {
        let result = format!("{:^width$}", "*".repeat(i * 2 - 1));
        println!("{}", result.trim_end());
    }
}
