use std::io;

const MAX: usize = 100_000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let [n, k] = [(); 2].map(|_| input());
    let mut memo = [0; MAX];

    for (i, (weight, value)) in (0..n).map(|_| (input(), input() as i32)).enumerate() {
        if i == 0 {
            for j in weight..=k {
                memo[j] = value;
            }

            continue;
        }

        let mut temp = [0; MAX];

        for (j, &max_val) in memo[..=k].iter().enumerate() {
            temp[j] = if j < weight {
                max_val
            } else {
                max_val.max(value + memo[j - weight])
            };
        }

        memo = temp;
    }

    println!("{}", memo[k]);
}
