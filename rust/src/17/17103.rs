fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n = parse_int(&buf);
    let preime_sieve = get_prime_sieve(1_000_000);

    for _ in 0..n {
        read_line(&mut buf);

        let num = parse_int(&buf);
        let count = get_goldbach_partition_count(num, &preime_sieve);

        println!("{count}");
    }
}

fn get_goldbach_partition_count(num: i32, prime_sieve: &Vec<bool>) -> i32 {
    if num == 4 {
        return 1;
    }

    let mut count = 0;
    let half = num / 2;
    let half = if half % 2 == 0 { half - 1 } else { half };

    let prime_nums = (3..=half).step_by(2).filter(|&n| prime_sieve[n as usize]);

    for a in prime_nums {
        let b = num - a;

        if prime_sieve[b as usize] {
            count += 1;
        }
    }

    count
}

fn get_prime_sieve(num: i32) -> Vec<bool> {
    let mut prime_sieve = vec![true; (num + 1) as usize];
    prime_sieve[0] = false;
    prime_sieve[1] = false;

    for i in 2..=(num as f64).sqrt() as usize {
        if !prime_sieve[i] {
            continue;
        }

        for j in ((i * i)..=num as usize).step_by(i) {
            prime_sieve[j] = false;
        }
    }

    prime_sieve
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
