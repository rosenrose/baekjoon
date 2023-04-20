use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, k, b) = (input(), input(), input());
    let mut is_broken = vec![false; n];

    for _ in 0..b {
        is_broken[input() - 1] = true;
    }

    let (mut count, mut min_count) = (0, usize::MAX);

    for (i, window) in is_broken.windows(k).enumerate() {
        if i == 0 {
            count = window.iter().filter(|&&b| b).count();
        } else {
            if window[k - 1] {
                count += 1;
            }
        }

        min_count = count.min(min_count);

        if window[0] {
            count -= 1;
        }
    }

    println!("{min_count}");
}
