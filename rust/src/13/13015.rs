use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();
    let n: usize = buf.trim().parse().unwrap();

    for i in (1..=n).chain((1..n).rev()) {
        let blank = if n == i { 0 } else { 2 * (n - i) - 1 };

        if i == 1 {
            let star = "*".repeat(n);
            writeln!(output, "{star}{:blank$}{star}", "").unwrap();
            continue;
        }

        let left = i - 1;
        let gap = n - 2;

        if i == n {
            writeln!(output, "{:left$}*{:gap$}*{:gap$}*", "", "", "").unwrap();
            continue;
        }

        writeln!(
            output,
            "{:left$}*{:gap$}*{:blank$}*{:gap$}*",
            "", "", "", ""
        )
        .unwrap();
    }

    print!("{output}");
}
