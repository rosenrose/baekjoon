fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let result = star(n).join("\n");

    print!("{result}");
}

fn star(n: usize) -> Vec<String> {
    if n == 3 {
        return vec![
            "  *  ".to_string(),
            " * * ".to_string(),
            "*****".to_string(),
        ];
    }

    let inner = star(n / 2);
    let inner_width = inner[0].len();

    let mut result = Vec::with_capacity(n);
    let blank = " ".repeat(inner_width / 2 + 1);

    for i in inner.iter() {
        result.push(format!("{blank}{i}{blank}"));
    }

    for i in inner {
        result.push(format!("{i} {i}"));
    }

    result
}
