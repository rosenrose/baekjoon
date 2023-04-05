fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut sum = 0;
    let mut n: Vec<_> = buf
        .trim()
        .chars()
        .map(|c| {
            sum += c as i32 - '0' as i32;
            c
        })
        .collect();

    if !n.contains(&'0') || sum % 3 != 0 {
        println!("-1");
        return;
    }

    n.sort_by_key(|&ch| std::cmp::Reverse(ch));

    println!("{}", String::from_iter(n));
}
