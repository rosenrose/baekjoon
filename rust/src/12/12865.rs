use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next().unwrap();

    let (n, k) = (input(), input() as usize);
    let mut memo = vec![0; k + 1];

    for i in 0..n {
        let (w, v) = (input() as usize, input());

        if i == 0 {
            for j in w..=k {
                memo[j] = v;
            }

            continue;
        }

        memo = memo
            .iter()
            .enumerate()
            .map(|(j, &max_val)| {
                if j < w {
                    max_val
                } else {
                    max_val.max(v + memo[j - w])
                }
            })
            .collect();
    }

    println!("{}", memo[k]);
}
