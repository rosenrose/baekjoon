fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    let result: Vec<String> = print(n);

    println!("{}", result.join("\n"));
}

fn print(n: usize) -> Vec<String> {
    if n == 3 {
        return vec![
            "  *  ".to_string(),
            " * * ".to_string(),
            "*****".to_string(),
        ];
    }

    let star = print(n / 2);
    let blank = vec![" ".repeat(n / 2); n / 2];
    let mut result = Vec::new();

    for i in 0..n / 2 {
        result.push(format!("{}{}{}", blank[i], star[i], blank[i]));
    }

    for i in 0..n / 2 {
        result.push(format!("{}{}{}", star[i], &blank[i][..1], star[i]));
    }

    result
}
