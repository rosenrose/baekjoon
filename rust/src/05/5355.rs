use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().skip(1) {
        let mut formula = input.split(' ');
        let mut num: f64 = formula.next().unwrap().parse().unwrap();

        for operator in formula {
            match operator {
                "@" => num *= 3.0,
                "%" => num += 5.0,
                "#" => num -= 7.0,
                _ => unreachable!(),
            };
        }

        println!("{num:.2}");
    }
}
