fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut sum = 0;
    let mut n: Vec<_> = buf
        .trim()
        .chars()
        .map(|c| {
            sum += c.to_digit(10).unwrap();
            c
        })
        .collect();

    if !n.contains(&'0') || sum % 3 != 0 {
        println!("-1");
        return;
    }

    n.sort_by(|a, b| b.cmp(a));

    println!("{}", String::from_iter(n));
}
