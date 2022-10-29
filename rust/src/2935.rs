fn main() {
    let mut buf = String::new();

    if let [a, operator, b] = &parse_str_vec_lines(&mut buf, 3)[..] {
        let (greater, less) = if a.len() > b.len() { (a, b) } else { (b, a) };

        let result = match &operator[..] {
            "+" => {
                if greater.len() == less.len() {
                    format!("2{}", "0".repeat(greater.len() - 1))
                } else {
                    format!("{}{}", &greater[..greater.len() - less.len()], less)
                }
            }
            "*" => format!("1{}", "0".repeat(greater.len() + less.len() - 2)),
            _ => String::new(),
        };

        println!("{result}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_str_vec_lines(buf: &mut String, n: i32) -> Vec<String> {
    (0..n)
        .map(|_| {
            read_line(buf);
            buf.trim().to_string()
        })
        .collect()
}
