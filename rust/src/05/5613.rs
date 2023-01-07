use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let mut lhs: i32 = input.next().unwrap().parse().unwrap();
    let mut op = "";

    for line in input {
        if let Ok(rhs) = line.parse::<i32>() {
            match op {
                "+" => lhs += rhs,
                "-" => lhs -= rhs,
                "*" => lhs *= rhs,
                "/" => lhs /= rhs,
                _ => (),
            }
        } else {
            op = line;
        }
    }

    println!("{lhs}");
}
