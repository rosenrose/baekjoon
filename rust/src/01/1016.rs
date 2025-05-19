use std::io;

const MAX: usize = 1_000_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let [min, max] = [(); 2].map(|_| input.next().unwrap());
    let max_num = (max as f64).sqrt() as usize;
    let prime_nums = get_prime_nums(max_num);
    let mut square_free_num_sieve = [true; MAX + 1];

    for p in prime_nums {
        let p_square = p * p;
        let start = p_square * (min / p_square);
        let start = if start < min { start + p_square } else { start };

        for square_num in (start..=max).step_by(p_square) {
            square_free_num_sieve[square_num - min] = false;
        }
    }

    println!(
        "{}",
        square_free_num_sieve[..max - min + 1]
            .iter()
            .filter(|&&s| s)
            .count()
    );
}

fn get_prime_nums(num: usize) -> Vec<usize> {
    let mut sieve = [true; MAX + 1];
    let mut prime_nums = Vec::new();

    for i in 2..=num {
        if sieve[i] {
            prime_nums.push(i);
        }

        for p in prime_nums.iter().take_while(|&&p| i * p <= num) {
            sieve[i * p] = false;

            if i % p == 0 {
                break;
            }
        }
    }

    prime_nums
}
