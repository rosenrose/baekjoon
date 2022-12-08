use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();

    let n: usize = buf.trim().parse().unwrap();
    let at = "@".repeat(n * 3);
    let blank = " ".repeat(n);

    for _ in 0..n {
        writeln!(output, "{at}{blank}{}", &at[..n]).unwrap();
    }
    for _ in 0..n * 3 {
        writeln!(output, "{}{blank}{}{blank}{}", &at[..n], &at[..n], &at[..n]).unwrap();
    }
    for _ in 0..n {
        writeln!(output, "{}{blank}{at}", &at[..n]).unwrap();
    }

    print!("{output}");
}
