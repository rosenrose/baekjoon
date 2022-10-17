use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let result: Vec<String> = print(n);

    writeln!(stdout, "{}", result.join("\n")).unwrap();
}

fn print(n: usize) -> Vec<String> {
    if n == 1 {
        return vec!["*".to_string()];
    }

    let inner = print(n - 1);
    let size = 4 * n - 3;

    let mut result = Vec::new();

    result.push("*".repeat(size));
    result.push(format!("*{:blank$}*", "", blank = size - 2));

    for i in 0..inner.len() {
        result.push(format!("* {} *", inner[i]));
    }

    result.push(format!("*{:blank$}*", "", blank = size - 2));
    result.push("*".repeat(size));

    result
}
