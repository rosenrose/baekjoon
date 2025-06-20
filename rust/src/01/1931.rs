use std::io;

const MAX: usize = 100_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut times = [[0; 2]; MAX];

    for time in &mut times[..n] {
        *time = [(); 2].map(|_| input.next().unwrap());
    }

    times[..n].sort_unstable_by_key(|&[start, end]| [end, start]);
    // println!("{times:?}");
    let mut count = 0;
    let mut prev_end = 0;

    for &[cur_start, cur_end] in &times[..n] {
        if prev_end <= cur_start {
            count += 1;
            prev_end = cur_end;
        }
    }

    println!("{count}");
}
