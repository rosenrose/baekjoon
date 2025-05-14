fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, k] = parse_int_vec(&buf)[..] else {
        return;
    };
    let mut deleted = 0;

    get_prime_sieve(n, k as i32, &mut deleted);

    println!("{deleted}");
}

fn get_prime_sieve(num: usize, mut k: i32, deleted: &mut usize) -> Vec<bool> {
    let mut prime_sieve = vec![true; num + 1];
    prime_sieve[0] = false;
    prime_sieve[1] = false;

    (2..=num).for_each(|i| {
        if !prime_sieve[i] {
            return;
        }

        for j in (i..=num).step_by(i) {
            if !prime_sieve[j] {
                continue;
            }

            prime_sieve[j] = false;
            k -= 1;

            if k == 0 {
                *deleted = j;
            }
        }
    });

    prime_sieve
}

// fn get_prime_sieve(num: usize) -> Vec<bool> {
//     let mut sieve = vec![true; num + 1];
//     (sieve[0], sieve[1]) = (false, false);

//     for i in (2..).take_while(|i| i * i <= num) {
//         if !sieve[i] {
//             continue;
//         }

//         for j in (i * i..=num).step_by(i) {
//             sieve[j] = false;
//         }
//     }

//     sieve
// }

fn parse_int_vec(buf: &str) -> Vec<usize> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
