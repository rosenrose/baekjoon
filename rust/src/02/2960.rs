use std::io;

const MAX: usize = 1000 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_whitespace().flat_map(str::parse::<usize>);

    let [n, k] = [(); 2].map(|_| input.next().unwrap());
    let mut deleted = 0;

    get_prime_sieve(n, k as i32, &mut deleted);

    println!("{deleted}");
}

fn get_prime_sieve(num: usize, mut k: i32, deleted: &mut usize) -> [bool; MAX] {
    let mut prime_sieve = [true; MAX];
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

// fn get_prime_sieve(num: usize) -> [bool; MAX] {
//     let mut sieve = [true; MAX];
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
