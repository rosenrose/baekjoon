use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for n in buf.lines().flat_map(str::parse::<i32>) {
        let (mut rem, mut pow_rem) = (1 % n, 1 % n);
        let mut count = 1;

        while rem != 0 {
            pow_rem = (pow_rem * (10 % n)) % n;
            rem = (rem + (pow_rem % n)) % n;

            count += 1;
        }

        println!("{count}");
    }
}
