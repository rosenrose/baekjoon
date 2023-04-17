fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    if n == 1 {
        println!("0");
        return;
    }

    let prime_nums = get_prime_nums(n as usize);
    let (mut start, mut end) = (0, 0);
    let mut sum = prime_nums[start];
    let mut count = 0;

    loop {
        if sum == n {
            count += 1;
        }

        if sum < n {
            end += 1;

            if end == prime_nums.len() {
                break;
            }

            sum += prime_nums[end];
        } else {
            sum -= prime_nums[start];
            start += 1;
        }
    }

    println!("{count}");
}

fn get_prime_nums(num: usize) -> Vec<i32> {
    let mut sieve = vec![true; num + 1];
    let mut prime_nums = Vec::new();

    for i in 2..=num {
        if sieve[i] {
            prime_nums.push(i as i32);
        }

        for &p in prime_nums.iter().take_while(|&&p| i * p as usize <= num) {
            sieve[i * p as usize] = false;

            if i as i32 % p == 0 {
                break;
            }
        }
    }

    prime_nums
}
