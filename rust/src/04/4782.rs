fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);

        if buf.trim() == "0 0" {
            return;
        }

        if let [b, n] = parse_int_vec(&buf)[..] {
            let step = n / get_gcd(b, n);

            for m in (step..=2 * n).rev().step_by(step).filter(|&m| m != n) {
                let dividend = b * m * (2 * n - m);
                let divisor = n * n;

                if dividend % divisor != 0 {
                    continue;
                }

                let a = dividend / divisor;
                print!("{a}/{m} ");
            }
        }

        println!("");
    }
}

fn get_gcd(mut a: usize, mut b: usize) -> usize {
    loop {
        (a, b) = (b, a % b);

        if b == 0 {
            return a;
        }
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
