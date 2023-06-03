fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut n: Vec<_> = buf.trim().chars().collect();
    let sum: i32 = n.iter().map(|&ch| (ch as u8 - b'0') as i32).sum();

    if !n.contains(&'0') || sum % 3 != 0 {
        println!("-1");
        return;
    }

    n.sort_unstable();

    println!("{}", String::from_iter(n.iter().rev()));
}
