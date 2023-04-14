use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<usize>);
    let mut output = String::new();

    const MAX: usize = 1000;
    let min_factors = get_min_factors(MAX);
    let mut euler_phi = vec![0; MAX + 1];

    for i in 2..=1000 {
        let mut num = i;
        let mut factors = HashSet::new();

        while num > 1 {
            factors.insert(min_factors[num]);
            num /= min_factors[num];
        }

        euler_phi[i] = factors.iter().fold(i, |acc, p| acc * (p - 1) / p);
    }

    for n in input.skip(1) {
        let count = 3 + (2..=n).map(|x| euler_phi[x] * 2).sum::<usize>();

        writeln!(output, "{count}").unwrap();
    }

    print!("{output}");
}

fn get_min_factors(num: usize) -> Vec<usize> {
    let mut min_factors: Vec<_> = (0..=num).collect();

    for i in (2..).take_while(|i| i * i <= num) {
        if min_factors[i] != i {
            continue;
        }

        for j in (i * i..=num).step_by(i) {
            if min_factors[j] != j {
                continue;
            }

            min_factors[j] = i;
        }
    }

    min_factors
}
