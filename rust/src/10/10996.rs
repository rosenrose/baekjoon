fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    if n == 1 {
        println!("*");
        return;
    }

    let mut result: String;

    for i in 1..=n * 2 {
        if n % 2 == 0 {
            result = (if i % 2 == 0 { " *" } else { "* " }).repeat(n / 2);
        } else {
            result = if i % 2 == 0 {
                " *".repeat(n / 2)
            } else {
                "* ".repeat((n + 1) / 2)
            };
        }

        println!("{}", result.trim_end());
    }
}
