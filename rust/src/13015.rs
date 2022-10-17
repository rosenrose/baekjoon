fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    for i in (1..=n).chain((1..n).rev()) {
        let blank = if n == i { 0 } else { 2 * (n - i) - 1 };

        if i == 1 {
            let star = "*".repeat(n);
            println!("{star}{:blank$}{star}", "");
            continue;
        }

        let left = i - 1;
        let gap = n - 2;

        if i == n {
            println!("{:left$}*{:gap$}*{:gap$}*", "", "", "");
            continue;
        }

        println!("{:left$}*{:gap$}*{:blank$}*{:gap$}*", "", "", "", "");
    }
}
