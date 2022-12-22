use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    print!("{}", star(n));
}

fn star(n: usize) -> String {
    if n == 1 {
        return "*".to_string();
    }

    let inner = star(n - 1);
    let size = 4 * n - 3;

    let mut result = String::new();
    let blank = " ".repeat(size - 2);

    writeln!(result, "{}", "*".repeat(size)).unwrap();
    writeln!(result, "*{blank}*").unwrap();

    for i in inner.lines() {
        writeln!(result, "* {i} *").unwrap();
    }

    writeln!(result, "*{blank}*").unwrap();
    writeln!(result, "{}", "*".repeat(size)).unwrap();

    result
}
