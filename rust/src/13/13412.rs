use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().map(|s| s.parse::<i32>().unwrap());

    for n in input.skip(1) {
        if n == 1 {
            println!("1");
            continue;
        }

        let pairs = (1..)
            .take_while(|i| i * i < n)
            .filter(|&i| {
                if n % i != 0 {
                    return false;
                }

                get_gcd(i, n / i) == 1
            })
            .count();

        println!("{pairs}");
    }
}

fn get_gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
