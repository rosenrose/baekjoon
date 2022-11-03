fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);

        let tokens = parse_str_vec(&buf);
        let mut num = tokens[0].parse::<f64>().unwrap();
        let operators = &tokens[1..];

        for &op in operators {
            match op {
                "@" => num *= 3.0,
                "%" => num += 5.0,
                "#" => num -= 7.0,
                _ => (),
            };
        }

        println!("{num:.2}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_str_vec(buf: &String) -> Vec<&str> {
    buf.split_whitespace().collect()
}
