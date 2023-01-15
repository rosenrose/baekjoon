use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let min_factors = get_min_factors(5_000_000);
    // println!("{min_factors:?}");

    for mut num in input.skip(1) {
        while num > 1 {
            let min_factor = min_factors[num as usize];
            num /= min_factor;

            write!(output, "{min_factor} ").unwrap();
        }

        writeln!(output, "").unwrap();
    }

    print!("{output}");
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
