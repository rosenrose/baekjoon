use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();
    let n: usize = buf.trim().parse().unwrap();

    for i in (1..=n).chain((1..n).rev()) {
        writeln!(output, "{:>width$}", "*".repeat(i), width = n).unwrap();
    }

    print!("{output}");
}
