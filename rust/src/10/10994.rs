use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let result = print(n);

    writeln!(stdout, "{}", result.join("\n")).unwrap();
}

fn print(n: usize) -> Vec<String> {
    if n == 1 {
        return vec!["*".to_string()];
    }

    let inner = print(n - 1);
    let size = 4 * n - 3;
    let blank = size - 2;

    let mut result = Vec::new();

    result.push("*".repeat(size));
    result.push(format!("*{:blank$}*", ""));

    for i in 0..inner.len() {
        result.push(format!("* {} *", inner[i]));
    }

    result.push(format!("*{:blank$}*", ""));
    result.push("*".repeat(size));

    result
}
