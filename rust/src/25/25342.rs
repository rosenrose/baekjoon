use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i64>);
    let mut output = String::new();

    for n in input.skip(1) {
        let lcm1 = get_lcm(n, n - 1);
        let lcm2 = get_lcm(n - 2, (n - 3).max(1));

        let lcm_list = [
            get_lcm(lcm1, n - 2),
            get_lcm(lcm1, (n - 3).max(1)),
            get_lcm(n, lcm2),
            get_lcm(n - 1, lcm2),
        ];

        writeln!(output, "{}", lcm_list.iter().max().unwrap()).unwrap();
    }

    print!("{output}");
}

fn get_lcm(a: i64, b: i64) -> i64 {
    a / get_gcd(a, b) * b
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
