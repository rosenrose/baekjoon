use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();

    let (_, formula) = (input.next(), input.next().unwrap());
    let nums: Vec<_> = input.flat_map(str::parse::<f64>).collect();

    let mut stack = Vec::new();

    for ch in formula.chars() {
        if matches!(ch, 'A'..='Z') {
            stack.push(nums[(ch as u8 - b'A') as usize]);
            continue;
        }

        let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
        let result = match ch {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => a / b,
            _ => unreachable!(),
        };

        stack.push(result);
    }

    println!("{:.2}", stack.pop().unwrap());
}
