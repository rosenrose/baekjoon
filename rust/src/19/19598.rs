use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let mut times: Vec<_> = (0..input())
        .flat_map(|_| {
            let (start, end) = (input(), input());
            [(start, true), (end, false)]
        })
        .collect();
    times.sort_unstable();

    let (mut count, mut max_count) = (0, 1);

    for (_, is_start) in times {
        if is_start {
            count += 1;
        } else {
            count -= 1;
        }

        max_count = count.max(max_count);
    }

    println!("{max_count}");
}
