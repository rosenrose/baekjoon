use std::io;

const MAX: usize = 10000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);

    let [m, n] = [(); 2].map(|_| input.next().unwrap());
    let mut prime_nums = [0; MAX];
    let mut prime_nums_len = 0;

    for num in (m..=n).filter(is_prime) {
        prime_nums[prime_nums_len] = num;
        prime_nums_len += 1;
    }

    if prime_nums_len == 0 {
        println!("-1");
        return;
    }

    println!(
        "{}\n{}",
        prime_nums[..prime_nums_len].iter().sum::<i32>(),
        prime_nums[..prime_nums_len].iter().min().unwrap()
    );
}

fn is_prime(num: &i32) -> bool {
    if *num <= 1 {
        return false;
    }

    (2..).take_while(|i| i * i <= *num).all(|i| *num % i != 0)
}
