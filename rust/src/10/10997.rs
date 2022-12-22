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

    let mut inner = star(n - 1);

    if n == 2 {
        writeln!(inner, "\n*").unwrap();
        writeln!(inner, "*").unwrap();
    }

    let width = 4 * n - 3;
    let mut result = String::new();
    let blank = " ".repeat(width - 2);

    writeln!(result, "{}", "*".repeat(width)).unwrap();
    writeln!(result, "*").unwrap();

    for (idx, i) in inner.lines().enumerate() {
        match idx {
            0 => writeln!(result, "* {i}**"),
            1 => writeln!(result, "* {i}{}*", &blank[..width - 4]),
            _ => writeln!(result, "* {i} *"),
        }
        .unwrap();
    }

    writeln!(result, "*{blank}*").unwrap();
    writeln!(result, "{}", "*".repeat(width)).unwrap();

    result
}
