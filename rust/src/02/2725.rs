use std::collections::HashSet;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let min_factors = get_min_factors(1000);
    let mut euler_phi = vec![0, 0];

    for mut i in 2..=1000 {
        let mut phi = i;
        let mut factors = HashSet::new();

        while i > 1 {
            factors.insert(min_factors[i]);
            i /= min_factors[i];
        }

        phi = factors.iter().fold(phi, |acc, p| acc * (p - 1) / p);
        euler_phi.push(phi);
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
