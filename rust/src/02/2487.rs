use std::collections::HashSet;
use std::io;

const MAX: usize = 20000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut shuffle = [0; MAX];

    for (i, num) in input.enumerate() {
        shuffle[i] = num - 1;
    }

    let mut cycles = [0_usize; MAX];

    for i in 0..n {
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

    let cycles: HashSet<_> = cycles[..n].iter().copied().collect();

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
