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
        writeln!(result, "***").unwrap();
        writeln!(result, "* *").unwrap();
        writeln!(result, "***").unwrap();

        return result;
    }

    let inner = star(n / 3);
    let inner_size = n / 3;

    let blank = " ".repeat(inner_size);

    for i in inner.lines() {
        writeln!(result, "{}", i.repeat(3)).unwrap();
    }

    for i in inner.lines() {
        writeln!(result, "{i}{blank}{i}").unwrap();
    }

    for i in inner.lines() {
        writeln!(result, "{}", i.repeat(3)).unwrap();
    }

    result
}
