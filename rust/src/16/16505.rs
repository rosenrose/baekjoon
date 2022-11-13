use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let result = print(n);

    for r in result {
        writeln!(stdout, "{}", r.trim_end()).unwrap();
    }
}

fn print(n: usize) -> Vec<String> {
    if n == 0 {
        return vec!["*".to_string()];
    }

    let inner = print(n - 1);
    let inner_height = inner.len();
    let inner_width = inner[0].len();

    let mut result = Vec::new();

    for i in 0..inner_height {
        result.push(format!("{}{}", inner[i], inner[i]));
    }
    for i in 0..inner_height {
        result.push(format!("{}{:inner_width$}", inner[i], ""));
    }

    result
}
