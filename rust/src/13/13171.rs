use std::io;

const M: i64 = 1_000_000_007;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i64>);

    let [a, x] = [(); 2].map(|_| input.next().unwrap());

    println!("{}", pow_rem(a, x));
}

fn pow_rem(base: i64, exp: i64) -> i64 {
    if exp == 1 {
        return base % M;
    }

    let mut rem = pow_rem(base, exp >> 1);
    rem = (rem * rem) % M;

    if exp & 1 == 0 {
        rem
    } else {
        (rem * (base % M)) % M
    }
}
