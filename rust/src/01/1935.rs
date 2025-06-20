use std::io;

const MAX: usize = 26;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut input = || input.next().unwrap();

    let n: usize = input().parse().unwrap();
    let formula = input();
    let mut nums = [0.0; MAX];

    for num in &mut nums[..n] {
        *num = input().parse().unwrap();
    }

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
