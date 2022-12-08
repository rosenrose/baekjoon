use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();

    let n: usize = buf.trim().parse().unwrap();
    let at = "@".repeat(n * 5);

    for _ in 0..2 {
        for _ in 0..n {
            writeln!(output, "{at}").unwrap();
        }
        for _ in 0..n {
            writeln!(output, "{}", &at[..n]).unwrap();
        }
    }
    for _ in 0..n {
        writeln!(output, "{at}").unwrap();
    }

    print!("{output}");
}
