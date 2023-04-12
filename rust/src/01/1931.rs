use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let mut times: Vec<_> = (0..input()).map(|_| (input(), input())).collect();
    times.sort_unstable_by_key(|&(start, end)| (end, start));
    // println!("{times:?}");
    let mut count = 0;
    let mut prev_end = 0;

    for (cur_start, cur_end) in times {
        if prev_end <= cur_start {
            count += 1;
            prev_end = cur_end;
        }
    }

    println!("{count}");
}
