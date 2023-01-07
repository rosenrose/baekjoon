use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut input = || input.next().unwrap();

    let (a, operator, b) = (input(), input(), input());
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
