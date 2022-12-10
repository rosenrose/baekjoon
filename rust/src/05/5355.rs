use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for input in buf.lines().skip(1) {
        let mut formula = input.split_whitespace();
        let mut num: f64 = formula.next().unwrap().parse().unwrap();

        for operator in formula {
            match operator {
                "@" => num *= 3.0,
                "%" => num += 5.0,
                "#" => num -= 7.0,
                _ => (),
            };
        }

        println!("{num:.2}");
    }
}
