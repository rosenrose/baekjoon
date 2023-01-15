use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut stack = Vec::new();

    for num in input.skip(1) {
        if num == 0 {
            stack.pop();
        } else {
            stack.push(num);
        }
    }

    println!("{}", stack.iter().sum::<i32>());
}
