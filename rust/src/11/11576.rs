use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (a, b) = (input.next().unwrap(), input.next().unwrap());
    input.next();

    let mut b_nums = Vec::new();
    let mut a_num: i32 = input
        .rev()
        .enumerate()
        .map(|(i, num)| num * a.pow(i as u32))
        .sum();

    while a_num != 0 {
        b_nums.push(a_num % b);
        a_num /= b;
    }

    for b_num in b_nums.iter().rev() {
        print!("{b_num} ");
    }
}
