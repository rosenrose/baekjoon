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

    for r in result {
        writeln!(stdout, "{}", r.trim_end()).unwrap();
    }
}

fn print(n: usize) -> Vec<String> {
    if n == 1 {
        return vec!["*".to_string()];
    }

    let inner = print(n - 1);
    let inner_size = inner.len();
    let inner_width = inner[0].len();

    let size = inner_size * 2 + 1;
    let width = inner_width + (size + 1);

    let horizontal = "*".repeat(width);
    let point = format!("{:^width$}", "*");
    let side = (2..=size - 1).map(|i| {
        let pad = size - i;
        let blank = i * 2 - 3;
        format!("{:pad$}*{:blank$}*{:pad$}", "", "", "")
    });

    let mut result: Vec<String>;
    let rest_width = (width - inner_width) / 2;

    if n % 2 == 0 {
        result = side.rev().collect();
        result.insert(0, horizontal);
        result.push(point);

        for i in 1..=inner_size {
            result[i] = format!(
                "{}{}{}",
                &result[i][..rest_width],
                inner[i - 1],
                &result[i][width - rest_width..]
            );
        }
    } else {
        result = side.collect();
        result.insert(0, point);
        result.push(horizontal);

        for i in inner_size..size - 1 {
            result[i] = format!(
                "{}{}{}",
                &result[i][..rest_width],
                inner[i - inner_size],
                &result[i][width - rest_width..]
            );
        }
    }

    result
}
