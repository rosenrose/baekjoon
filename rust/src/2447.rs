fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    let result: Vec<String> = print(n);

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

    let star = print(n / 3);
    let blank = vec![" ".repeat(n / 3); n / 3];
    let mut result = Vec::new();

    for i in 0..n / 3 {
        result.push(star[i].repeat(3));
    }

    for i in 0..n / 3 {
        result.push(format!("{}{}{}", star[i], blank[i], star[i]));
    }

    for i in 0..n / 3 {
        result.push(result[i].clone());
    }

    result
}
