use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut output = String::new();

    let n: usize = buf.trim().parse().unwrap();
    let result = star(n);

    for r in result {
        writeln!(output, "{}", r.trim_end()).unwrap();
    }

    print!("{output}");
}

fn star(n: usize) -> Vec<String> {
    if n == 0 {
        return vec!["*".to_string()];
    }

    let inner = star(n - 1);
    let inner_width = inner[0].len();

    let mut result = Vec::new();
    let blank = " ".repeat(inner_width);

    for i in inner.iter() {
        result.push(format!("{i}{i}"));
    }
    for i in inner {
        result.push(format!("{i}{blank}"));
    }

    result
}
