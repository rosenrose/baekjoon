use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();

    let n: usize = buf.trim().parse().unwrap();
    let at = "@".repeat(n * 5);
    let blank = " ".repeat(n * 3);

    for _ in 0..n {
        writeln!(output, "{at}").unwrap();
    }
    for _ in 0..n * 3 {
        writeln!(output, "{}{blank}{}", &at[..n], &at[..n]).unwrap();
    }
    for _ in 0..n {
        writeln!(output, "{at}").unwrap();
    }

    print!("{output}");
}
