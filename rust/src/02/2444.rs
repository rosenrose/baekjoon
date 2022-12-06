use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();
    let n: usize = buf.trim().parse().unwrap();
    let width = n * 2 - 1;

    for i in (1..=n).chain((1..n).rev()) {
        let result = format!("{:^width$}", "*".repeat(i * 2 - 1));

        writeln!(output, "{}", result.trim_end()).unwrap();
    }

    print!("{output}");
}
