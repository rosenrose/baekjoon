fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        read_line(&mut buf);

        let mut operators = buf.split_whitespace();
        let mut num: f64 = operators.next().unwrap().parse().unwrap();

        for op in operators {
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
