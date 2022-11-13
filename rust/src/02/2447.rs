fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let result = print(n);

    println!("{}", result.join("\n"));
}

fn print(n: usize) -> Vec<String> {
    if n == 3 {
        #[rustfmt::skip]
        return vec![
            "***".to_string(),
            "* *".to_string(),
            "***".to_string()
          ];
    }

    let inner = print(n / 3);
    let inner_size = n / 3;

    let mut result = Vec::new();

    for i in 0..inner_size {
        result.push(inner[i].repeat(3));
    }

    for i in 0..inner_size {
        result.push(format!("{}{:inner_size$}{}", inner[i], "", inner[i]));
    }

    for i in 0..inner_size {
        result.push(inner[i].repeat(3));
    }

    result
}
