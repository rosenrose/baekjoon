use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let queuestack_count = input.next().unwrap() as usize;
    let queue_or_stack: Vec<_> = input.by_ref().take(queuestack_count).collect();
    let elements: Vec<_> = input
        .by_ref()
        .take(queuestack_count)
        .enumerate()
        .filter_map(|(i, num)| (queue_or_stack[i] == 0).then_some(num))
        .collect();
    let insert_count = input.next().unwrap() as usize;

    for num in elements.into_iter().rev().chain(input).take(insert_count) {
        write!(output, "{num} ").unwrap();
    }

    print!("{output}");
}
