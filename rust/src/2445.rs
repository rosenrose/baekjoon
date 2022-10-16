fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    for i in (1..=n).chain((1..n).rev()) {
        let star = "*".repeat(i);
        println!("{star:n$}{star:>n$}");
    }
}
