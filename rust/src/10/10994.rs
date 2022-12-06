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

    let inner = star(n - 1);
    let size = 4 * n - 3;

    let mut result = Vec::with_capacity(size);
    let blank = " ".repeat(size - 2);

    result.push("*".repeat(size));
    result.push(format!("*{blank}*"));

    for i in inner {
        result.push(format!("* {i} *"));
    }

    result.push(format!("*{blank}*"));
    result.push("*".repeat(size));

    result
}
