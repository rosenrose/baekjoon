use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for n in buf.lines().flat_map(str::parse::<i32>) {
        let (mut remainder, mut exp_remainder) = (1 % n, 1 % n);
        let mut count = 1;

        while remainder != 0 {
            exp_remainder = ((exp_remainder % n) * (10 % n)) % n;
            remainder = ((remainder % n) + (exp_remainder % n)) % n;

            count += 1;
        }

        println!("{count}");
    }
}
