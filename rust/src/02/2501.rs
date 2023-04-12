use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let (n, k) = (input.next().unwrap(), input.next().unwrap());
    let mut divisors = Vec::new();

    for i in (1..).take_while(|i| i * i <= n) {
        if n % i != 0 {
            continue;
        }

        divisors.push(i);

        if i != n / i {
            divisors.push(n / i);
        }
    }

    if divisors.len() < k {
        println!("0");
        return;
    }

    println!("{}", divisors.select_nth_unstable(k - 1).1);
}
