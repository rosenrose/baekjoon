use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let mut sum_accum = vec![0, 1];
    sum_accum.extend((1..=1_000_000).scan(1, |acc, mut num| {
        let mut count = 0;

        while num > 0 {
            if num % 10 == 0 {
                count += 1;
            }

            num /= 10;
        }

        *acc += count;
        Some(*acc)
    }));

    for (n, m) in (0..input()).map(|_| (input(), input())) {
        println!("{}", sum_accum[m + 1] - sum_accum[n]);
    }
}
