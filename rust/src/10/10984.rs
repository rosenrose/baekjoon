fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(&buf);

    for _ in 0..n {
        read_line(&mut buf);

        let classes = parse_int(&buf);
        let mut credits = 0;

        let sum: f64 = (0..classes)
            .map(|_| {
                read_line(&mut buf);

                match parse_float_vec(&buf)[..] {
                    [c, g] => {
                        credits += c as i32;
                        c * g
                    }
                    _ => 0.0,
                }
            })
            .sum();

        let gpa = sum / credits as f64;

        println!("{credits} {gpa:.1}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}

fn parse_float_vec(buf: &String) -> Vec<f64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
