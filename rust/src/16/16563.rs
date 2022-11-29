use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin, mut stdout) = (stdin.lock(), BufWriter::new(stdout.lock()));

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let min_factors = get_min_factors(5_000_000);
    // println!("{min_factors:?}");

    buf.lines()
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .for_each(|mut num| {
            while num > 1 {
                let min_factor = min_factors[num as usize];

                num /= min_factor;
                write!(stdout, "{min_factor} ").unwrap();
            }

            writeln!(stdout, "").unwrap();
        });
}

fn get_min_factors(num: usize) -> Vec<i32> {
    let mut min_factors: Vec<_> = (0..=num as i32).collect();

    for i in (2..).take_while(|i| i * i <= num) {
        if min_factors[i] != i as i32 {
            continue;
        }

        for j in (i * i..=num).step_by(i) {
            if min_factors[j] != j as i32 {
                continue;
            }

            min_factors[j] = i as i32;
        }
    }

    min_factors
}
