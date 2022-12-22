use std::fmt::Write;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    print!("{}", star(n));
}

fn star(n: usize) -> String {
    let mut result = String::new();

    if n == 3 {
        writeln!(result, "  *  ").unwrap();
        writeln!(result, " * * ").unwrap();
        writeln!(result, "*****").unwrap();

        return result;
    }

    let inner = star(n / 2);
    let inner_width = inner.lines().nth(0).unwrap().len();

    let blank = " ".repeat(inner_width / 2 + 1);

    for i in inner.lines() {
        writeln!(result, "{blank}{i}{blank}").unwrap();
    }

    for i in inner.lines() {
        writeln!(result, "{i} {i}").unwrap();
    }

    result
}
