fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let (k1, op1, k2, op2, k3) = (
        parse_int(input()),
        input(),
        parse_int(input()),
        input(),
        parse_int(input()),
    );

    #[rustfmt::skip]
    let (first, second) = (0..2).fold((k1, k2), |(f, s), i| {
        (
            match if i == 0 { op1 } else { op2 } {
                "+" => f + if i == 0 { k2 } else { k3 },
                "-" => f - if i == 0 { k2 } else { k3 },
                "*" => f * if i == 0 { k2 } else { k3 },
                "/" => f / if i == 0 { k2 } else { k3 },
                _ => Default::default(),
            },
            match if i == 0 { op2 } else { op1 } {
                "+" => if i == 0 { s + k3 } else { k1 + s },
                "-" => if i == 0 { s - k3 } else { k1 - s },
                "*" => if i == 0 { s * k3 } else { k1 * s },
                "/" => if i == 0 { s / k3 } else { k1 / s },
                _ => Default::default(),
            },
        )
    });

    println!("{}\n{}", first.min(second), first.max(second));
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
