fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let result = star(n).join("\n");

    print!("{result}");
}

fn star(n: usize) -> Vec<String> {
    if n == 3 {
        #[rustfmt::skip]
        return vec![
            "***".to_string(),
            "* *".to_string(),
            "***".to_string()
          ];
    }

    let inner = star(n / 3);
    let inner_size = n / 3;

    let mut result = Vec::with_capacity(n);
    let blank = " ".repeat(inner_size);

    for i in inner.iter() {
        result.push(i.repeat(3));
    }

    for i in inner.iter() {
        result.push(format!("{i}{blank}{i}"));
    }

    for i in inner {
        result.push(i.repeat(3));
    }

    result
}
