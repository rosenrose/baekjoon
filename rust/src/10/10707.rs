use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    if let [a, b, c, d, p] = input.collect::<Vec<_>>()[..] {
        let x_bill = a * p;
        let mut y_bill = b;

        if p > c {
            y_bill += (p - c) * d;
        }

        println!("{}", x_bill.min(y_bill));
    }
}
