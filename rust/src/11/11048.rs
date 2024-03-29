use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut memo = vec![0; m];

    for _ in 0..n {
        let mut next = vec![0; m];

        for c in 0..m {
            next[c] = input()
                + next[c.saturating_sub(1)]
                    .max(memo[c])
                    .max(memo[c.saturating_sub(1)]);
        }

        memo = next;
    }

    println!("{}", memo[m - 1]);
}
