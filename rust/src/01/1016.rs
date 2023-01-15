fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [min, max] = parse_int_vec(&buf)[..] else { return };
    let prime_nums = get_prime_nums((max as f64).sqrt() as usize);
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

fn get_prime_nums(num: usize) -> Vec<i64> {
    let mut sieve = vec![true; num + 1];
    let mut prime_nums = Vec::new();

    for i in 2..=num {
        if sieve[i] {
            prime_nums.push(i as i64);
        }

        for &p in prime_nums.iter().take_while(|&&p| i * p as usize <= num) {
            sieve[i * p as usize] = false;

            if i as i64 % p == 0 {
                break;
            }
        }
    }

    prime_nums
}

fn parse_int_vec(buf: &String) -> Vec<i64> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
