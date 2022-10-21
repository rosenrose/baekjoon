fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let at = "@".repeat(n + 2);
    let blank = n;

    println!("{at}");
    for _ in 2..n + 2 {
        println!("@{:blank$}@", "");
    }
    println!("{at}");
}
