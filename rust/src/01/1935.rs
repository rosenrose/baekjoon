fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();
    read_line(&mut buf);

    let formula = buf.trim().to_string();
    let nums = parse_float_vec_lines(&mut buf, n);
    let offset = 'A' as u8;

    let mut stack = Vec::new();

    for c in formula.chars() {
        match c {
            'A'..='Z' => stack.push(nums[(c as u8 - offset) as usize]),
            op => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();

                let result = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => a / b,
                    _ => 0.0,
                };

                stack.push(result);
            }
        }
    }

    println!("{:.2}", stack.pop().unwrap());
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_float(buf: &String) -> f64 {
    buf.trim().parse().unwrap()
}

fn parse_float_vec_lines(buf: &mut String, n: i32) -> Vec<f64> {
    (0..n)
        .map(|_| {
            read_line(buf);
            parse_float(buf)
        })
        .collect()
}
