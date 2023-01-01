use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    let (n, t) = (input.next().unwrap(), input.next().unwrap());
    let mut time = 0;
    let mut tasks: Vec<_> = input.rev().collect();

    while let Some(&task) = tasks.last() {
        if (t - time) < task {
            break;
        }

        time += tasks.pop().unwrap();
    }

    println!("{}", n - tasks.len());
}
