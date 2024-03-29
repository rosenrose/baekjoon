use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let [_, left, right] = [(); 3].map(|_| input.next().unwrap());
    let mut count = 0;
    let mut lcm_accum = vec![-1_i64];

    for num in input {
        for i in 0..lcm_accum.len() {
            let (lcm, sign) = (lcm_accum[i].abs(), lcm_accum[i].signum() * -1);
            let lcm = get_lcm(lcm, num);

            if lcm > right {
                continue;
            }

            count += ((right / lcm) - ((left - 1) / lcm)) * sign;
            lcm_accum.push(lcm * sign);
        }
    }

    println!("{count}");
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
