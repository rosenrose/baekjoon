use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    let result: Vec<String> = print(n);

    writeln!(stdout, "{}", result.join("\n")).unwrap();
}

fn print(n: usize) -> Vec<String> {
    if n == 3 {
        return vec![
            "  *  ".to_string(),
            " * * ".to_string(),
            "*****".to_string(),
        ];
    }

    let inner = print(n / 2);
    let inner_size = n / 2;
    let mut result = Vec::new();

    for i in 0..inner_size {
        result.push(format!("{:inner_size$}{}{:inner_size$}", "", inner[i], ""));
    }

    for i in 0..inner_size {
        result.push(format!("{} {}", inner[i], inner[i]));
    }

    result
}
