fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let [min, max] = parse_int_vec(&buf)[..] {
        let prime_sieve = get_prime_sieve((max as f64).sqrt() as usize);
        let prime_nums =
            prime_sieve
                .iter()
                .enumerate()
                .filter_map(|(i, &is_prime)| if is_prime { Some(i as i64) } else { None });

        let mut square_free_num_sieve = vec![true; (max - min + 1) as usize];

        for p in prime_nums {
            let p_square = p * p;
            let start = p_square * (min / p_square);
            let start = if start < min { start + p_square } else { start };

            for square_num in (start..=max).step_by(p_square as usize) {
                square_free_num_sieve[(square_num - min) as usize] = false;
            }
        }

        println!("{}", square_free_num_sieve.iter().filter(|&&s| s).count());
    }
}

fn get_prime_sieve(num: usize) -> Vec<bool> {
    let mut sieve = vec![true; num + 1];
    (sieve[0], sieve[1]) = (false, false);

    for i in (2..).take_while(|i| i * i <= num) {
        if !sieve[i] {
            continue;
        }

        for j in ((i * i)..=num).step_by(i) {
            sieve[j] = false;
        }
    }

    sieve
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
