use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (n, k) = (input(), input() as usize);
    let mut memo = vec![0; k + 1];

    for (i, (weight, value)) in (0..n).map(|i| (i, (input() as usize, input()))) {
        if i == 0 {
            for j in weight..=k {
                memo[j] = value;
            }

            continue;
        }

        memo = memo
            .iter()
            .enumerate()
            .map(|(j, &max_val)| {
                if j < weight {
                    max_val
                } else {
                    max_val.max(value + memo[j - weight])
                }
            })
            .collect();
    }

    println!("{}", memo[k]);
}
