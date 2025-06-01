use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let [a, b, c] = [(); 3].map(|_| input.next().unwrap());

    println!("{}", pow_rem(a, b, c));
}

fn pow_rem(base: i64, exp: i64, m: i64) -> i64 {
    if exp == 1 {
        return base % m;
    }

    let mut rem = pow_rem(base, exp >> 1, m);
    rem = (rem * rem) % m;

    if exp & 1 == 0 {
        rem
    } else {
        (rem * (base % m)) % m
    }
}
