use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines();
    input.next();

    let formula = input.next().unwrap();
    let nums: Vec<f64> = input.map(|s| s.parse().unwrap()).collect();

    let offset = 'A' as u8;
    let mut stack = Vec::new();

    for ch in formula.chars() {
        if matches!(ch, 'A'..='Z') {
            stack.push(nums[(ch as u8 - offset) as usize]);
            continue;
        }

        let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
        let result = match ch {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => a / b,
            _ => 0.0,
        };

        stack.push(result);
    }

    println!("{:.2}", stack.pop().unwrap());
}
