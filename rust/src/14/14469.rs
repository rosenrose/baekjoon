use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let mut times: Vec<_> = (0..input())
        .map(|_| {
            let start = input();
            let end = start + input();

            (start, end)
        })
        .collect();

    times.sort();
    let mut min_time = times[0].1;

    for &(start, end) in &times[1..] {
        if min_time <= start {
            min_time = end;
        } else {
            min_time += end - start;
        }
    }

    println!("{min_time}");
}
