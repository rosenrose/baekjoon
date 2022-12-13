fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let mut count = 0;
    let min_factors = get_min_factors(n);

    for i in (1..).take_while(|i| i * i <= n) {
        // a+b = i + 1 , a+b = n/i + 1
        if n % i != 0 {
            continue;
        }

        // count += (1..=(i + 1) / 2)
        //     .filter(|&a| get_gcd(a, i + 1 - a) == 1)
        //     .count();

        // if i == n / i {
        //     continue;
        // }

        // count += (1..=(n / i + 1) / 2)
        //     .filter(|&a| get_gcd(a, n / i + 1 - a) == 1)
        //     .count();

        count += (1..=(i + 1) / 2)
            .filter(|&a| {
                let mut a = a;
                let b = i + 1 - a;

                while a > 1 {
                    if b % min_factors[a] == 0 {
                        return false;
                    }

                    a /= min_factors[a];
                }

                true
            })
            .count();

        if i == n / i {
            continue;
        }

        count += (1..=(n / i + 1) / 2)
            .filter(|&a| {
                let mut a = a;
                let b = n / i + 1 - a;

                while a > 1 {
                    if b % min_factors[a] == 0 {
                        return false;
                    }

                    a /= min_factors[a];
                }

                true
            })
            .count();
    }

    println!("{count}");
}

fn get_min_factors(num: usize) -> Vec<usize> {
    let mut min_factors: Vec<_> = (0..=num).collect();

    for i in (2..).take_while(|i| i * i <= num) {
        if min_factors[i] != i {
            continue;
        }

        for j in (i * i..=num).step_by(i) {
            if min_factors[j] != j {
                continue;
            }

            min_factors[j] = i;
        }
    }

    min_factors
}

fn get_gcd(mut a: usize, mut b: usize) -> usize {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
