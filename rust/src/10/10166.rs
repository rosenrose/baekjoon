use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [d1, d2] = parse_int_vec(&buf)[..] else { return };
    let euler_phi = get_euler_phi(d2 as usize);
    let mut checked = vec![false; (d2 + 1) as usize];
    let mut count = 0;

    for mut d in d1..=d2 {
        let mut divisors = Vec::new();

        for i in (1..).take_while(|i| i * i <= d) {
            if d % i != 0 {
                continue;
            }

            divisors.push(i);

            if i != d / i {
                divisors.push(d / i);
            }
        }

        for divisor in divisors {
            let div = divisor as usize;

            if checked[div] {
                d -= euler_phi[div];
            }

            checked[div] = true;
        }

        count += d;
    }

    println!("{count}");
}

fn get_euler_phi(num: usize) -> Vec<i32> {
    let min_factors = get_min_factors(num);
    let mut euler_phi = vec![0, 1];

    for mut i in 2..=num {
        let mut phi = i as i32;
        let mut factors = HashSet::new();

        while i > 1 {
            factors.insert(min_factors[i]);
            i /= min_factors[i] as usize;
        }

        phi = factors.iter().fold(phi, |acc, &p| acc * (p - 1) / p);
        euler_phi.push(phi);
    }

    euler_phi
}

fn get_min_factors(num: usize) -> Vec<i32> {
    let mut min_factors: Vec<_> = (0..=num as i32).collect();

    for i in (2..).take_while(|i| i * i <= num) {
        if min_factors[i] != i as i32 {
            continue;
        }

        for j in (i * i..=num).step_by(i) {
            if min_factors[j] != j as i32 {
                continue;
            }

            min_factors[j] = i as i32;
        }
    }

    min_factors
}

fn parse_int_vec(buf: &String) -> Vec<i32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
