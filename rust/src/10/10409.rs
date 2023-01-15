use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

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
