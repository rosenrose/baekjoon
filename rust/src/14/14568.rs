fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let mut count = 0;

    for a in (2..n).step_by(2) {
        for b in 1..n - a {
            for c in b + 2..=n - a - b {
                if a + b + c != n {
                    continue;
                }

                count += 1;
            }
        }
    }

    println!("{count}");
}
