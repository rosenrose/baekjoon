use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [d1, d2] = parse_int_vec(&buf)[..] {
        let seats: HashSet<_> = (d1..=d2)
            .flat_map(|d| {
                let angle = (360, d);

                (0..d).map(move |i| {
                    let gcd = get_gcd(angle.0 * i, angle.1);
                    (angle.0 * i / gcd, angle.1 / gcd)
                })
            })
            .collect();

        println!("{}", seats.len());
    }
}

fn get_gcd(mut a: usize, mut b: usize) -> usize {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}

fn parse_int_vec(buf: &String) -> Vec<usize> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
