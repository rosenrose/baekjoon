use std::io;

const MAX: usize = 100_000 * 2;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut times = [Default::default(); MAX];

    for i in (0..n).map(|i| i << 1) {
        let [_, start, end] = [(); 3].map(|_| input.next().unwrap());

        times[i] = (start, true);
        times[i + 1] = (end, false);
    }

    times[..n * 2].sort_unstable();

    let (mut count, mut max_count) = (0, 1);

    for &(_, is_start) in &times[..n * 2] {
        if is_start {
            count += 1;
        } else {
            count -= 1;
        }

        max_count = count.max(max_count);
    }

    println!("{max_count}");
}
