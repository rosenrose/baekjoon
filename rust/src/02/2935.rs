use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines();
    let (a, operator, b) = (
        input.next().unwrap(),
        input.next().unwrap(),
        input.next().unwrap(),
    );

    let (greater, less) = if a.len() > b.len() { (a, b) } else { (b, a) };

    let result = match operator {
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
