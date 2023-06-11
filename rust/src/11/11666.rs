use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (n, lock_time) = (input() as usize, input());
    let mut times: Vec<_> = (0..n)
        .map(|_| {
            let start = input();
            let end = start + input();
            (start, end)
        })
        .collect();
    times.sort_unstable();

    let mut using_workstations = BinaryHeap::with_capacity(n >> 1);
    let mut unlock_count = 0;

    'outer: for (start, end) in times {
        while let Some(&Reverse(occupied_end)) = using_workstations.peek() {
            if start < occupied_end {
                break;
            }

            using_workstations.pop();

            if start <= occupied_end + lock_time {
                using_workstations.push(Reverse(end));
                continue 'outer;
            }
        }

        unlock_count += 1;
        using_workstations.push(Reverse(end));
    }

    println!("{}", n - unlock_count);
}
