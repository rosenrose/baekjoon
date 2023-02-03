use std::collections::HashSet;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap() - 1);

    let shuffle: Vec<_> = input.skip(1).collect();
    let mut cycles = vec![0; shuffle.len()];

    for i in 0..cycles.len() {
        if cycles[i] != 0 {
            continue;
        }

        let (mut next, mut count) = (shuffle[i], 1);
        let mut nodes = vec![i];

        while next != i {
            nodes.push(next);

            next = shuffle[next];
            count += 1;
        }

        for j in nodes {
            cycles[j] = count;
        }
    }

    let cycles: HashSet<_> = cycles.into_iter().collect();

    println!("{}", get_lcm(cycles.into_iter()));
}

fn get_lcm(nums: impl Iterator<Item = usize>) -> usize {
    nums.reduce(|a, b| a / get_gcd(a, b) * b).unwrap()
}

fn get_gcd(mut a: usize, mut b: usize) -> usize {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
