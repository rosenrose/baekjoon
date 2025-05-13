use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let queuestack_count = input.next().unwrap() as usize;
    let queue_or_stack = input.by_ref().take(queuestack_count).collect::<Vec<_>>();
    let elements = input
        .by_ref()
        .take(queuestack_count)
        .enumerate()
        .filter_map(|(i, num)| (queue_or_stack[i] == 0).then_some(num))
        .collect::<Vec<_>>();
    let inserts = input.skip(1).collect::<Vec<_>>();

    for num in elements.iter().rev().take(inserts.len()).chain(
        inserts
            .iter()
            .take(inserts.len().saturating_sub(elements.len())),
    ) {
        write!(output, "{num} ").unwrap();
    }

    print!("{output}");
}
