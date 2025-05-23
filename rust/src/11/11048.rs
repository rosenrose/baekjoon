use std::io;

const MAX: usize = 1000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut memo = [0; MAX];

    for _ in 0..n {
        let mut next = [0; MAX];

        for c in 0..m {
            next[c] = input.next().unwrap()
                + next[c.saturating_sub(1)]
                    .max(memo[c])
                    .max(memo[c.saturating_sub(1)]);
        }

        memo = next;
    }

    println!("{}", memo[m - 1]);
}
