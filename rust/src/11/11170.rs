use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    const MAX: usize = 1_000_000;
    let mut sum_accum = vec![0; MAX + 1];
    sum_accum[0] = 1;

    for i in 1..=MAX {
        let mut num = i;
        let mut count = 0;

        while num > 0 {
            if num % 10 == 0 {
                count += 1;
            }

            num /= 10;
        }

        sum_accum[i] = sum_accum[i - 1] + count;
    }

    for (n, m) in (0..input()).map(|_| (input(), input())) {
        println!(
            "{}",
            sum_accum[m] - if n == 0 { 0 } else { sum_accum[n - 1] }
        );
    }
}
