use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut times: Vec<_> = input.skip(1).collect();
    times.sort();

    let waits: i32 = times
        .iter()
        .scan(0, |acc, time| {
            *acc += time;
            Some(*acc)
        })
        .sum();

    println!("{waits}");
}
