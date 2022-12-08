use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();

    let n: usize = buf.trim().parse().unwrap();
    let at = "@".repeat(n * 3);
    let blank = " ".repeat(n * 3);

    for i in 0..n * 2 {
        let blank_len = (3 - (i / n)) * n;
        writeln!(output, "{}{}{}", &at[..n], &blank[..blank_len], &at[..n]).unwrap();
    }
    for _ in 0..n {
        writeln!(output, "{at}").unwrap();
    }
    for i in 0..n * 2 {
        let blank_len = ((i / n) + 2) * n;
        writeln!(output, "{}{}{}", &at[..n], &blank[..blank_len], &at[..n]).unwrap();
    }

    print!("{output}");
}
