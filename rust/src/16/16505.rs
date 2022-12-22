use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    print!("{}", star(n));
}

fn star(n: usize) -> String {
    if n == 0 {
        return "*".to_string();
    }

    let inner = star(n - 1);
    let inner_width = inner.lines().nth(0).unwrap().len();

    let mut result = String::new();
    let blank = " ".repeat(inner_width - 1);

    for (idx, i) in inner.lines().enumerate() {
        writeln!(result, "{i}{}{i}", &blank[..idx]).unwrap();
    }
    for i in inner.lines() {
        writeln!(result, "{i}").unwrap();
    }

    result
}
