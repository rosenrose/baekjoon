use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, k) = (input(), input());
    let mut medal_ranks = vec![([0; 3], 1); n + 1];

    for _ in 0..n {
        let country = input();
        medal_ranks[country].0 = [input(), input(), input()];
    }

    let (k_medal, _) = medal_ranks[k];

    for i in 1..=n {
        let (other_medal, _) = medal_ranks[i];

        if k_medal < other_medal {
            medal_ranks[k].1 += 1;
        }
    }

    println!("{}", medal_ranks[k].1);
}
