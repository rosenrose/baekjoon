use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let result: Vec<String> = print(n);

    for r in result {
        writeln!(stdout, "{}", r.trim_end()).unwrap();
    }
}

fn print(n: usize) -> Vec<String> {
    if n == 1 {
        return vec!["*".to_string()];
    }

    let mut inner = print(n - 1);
    if n == 2 {
        inner.push("*".to_string());
        inner.push("*".to_string());
    }

    let width = 4 * n - 3;
    let mut result = Vec::new();

    result.push("*".repeat(width));
    result.push(format!("{:width$}", "*"));

    for i in 0..inner.len() {
        result.push(format!(
            "* {}{}",
            inner[i],
            if i == 0 { "**" } else { " *" }
        ));
    }

    result.push(format!("*{:blank$}*", "", blank = width - 2));
    result.push("*".repeat(width));

    result
}
