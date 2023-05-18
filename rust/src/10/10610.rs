fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut n = buf.trim().as_bytes().to_vec();
    let sum: i32 = n.iter().map(|ch| (ch - b'0') as i32).sum();

    if !n.contains(&b'0') || sum % 3 != 0 {
        println!("-1");
        return;
    }

    n.sort_by_key(|&ch| std::cmp::Reverse(ch));

    println!("{}", String::from_utf8(n).unwrap());
}
