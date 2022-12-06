fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let result = star(n).join("\n");

    print!("{result}");
}

fn star(n: usize) -> Vec<String> {
    if n == 1 {
        return vec!["*".to_string()];
    }

    let mut inner = star(n - 1);

    if n == 2 {
        inner.push("*".to_string());
        inner.push("*".to_string());
    }

    let width = 4 * n - 3;
    let mut result = Vec::with_capacity(4 * n - 1);
    let blank = " ".repeat(width - 2);

    result.push("*".repeat(width));
    result.push("*".to_string());

    for (idx, i) in inner.iter().enumerate() {
        result.push(match idx {
            0 => format!("* {i}**",),
            1 => format!("* {i}{}*", &blank[..width - 4]),
            _ => format!("* {i} *"),
        });
    }

    result.push(format!("*{blank}*"));
    result.push("*".repeat(width));

    result
}
