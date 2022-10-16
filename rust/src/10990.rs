fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let width = n * 2 - 1;

    for i in 1..=n {
        if i == 1 {
            let result = format!("{:^width$}", "*");
            println!("{}", result.trim_end());
            continue;
        }

        println!("{:>left$}{}*", "*", " ".repeat(i * 2 - 3), left = n - i + 1);
    }
}
