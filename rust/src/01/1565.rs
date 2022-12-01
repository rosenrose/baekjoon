use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let d = parse_int_vec(&buf);
    read_line(&mut buf);

    let m = parse_int_vec(&buf);
    let gcd = get_gcd(m.into_iter());

    let mut lcm = 1;

    for num in d {
        lcm = get_lcm(lcm, num);

        if lcm > gcd {
            println!("0");
            return;
        }
    }

    let count = (1..)
        .take_while(|i| i * i <= gcd)
        .fold(HashSet::new(), |mut acc, i| {
            if gcd % i != 0 {
                return acc;
            }

            if i % lcm == 0 {
                acc.insert(i);
            }
            if (gcd / i) % lcm == 0 {
                acc.insert(gcd / i);
            }

            acc
        })
        .len();

    println!("{count}");
}

fn get_lcm(a: i64, b: i64) -> i64 {
    a / get_gcd([a, b].into_iter()) * b
}

fn get_gcd<I>(nums: I) -> i64
where
    I: Iterator<Item = i64>,
{
    nums.reduce(|mut a, mut b| loop {
        (a, b) = (b, a % b);

        if b == 0 {
            return a;
        }
    })
    .unwrap()
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
