use std::io;

const MAX: usize = 15 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    if n % 2 == 1 {
        println!("0");
        return;
    }

    let mut memo = [0; MAX];
    (memo[0], memo[1]) = (1, 3);

    for i in 2..=n / 2 {
        memo[i] = memo[i - 1] * 3 + (memo[0..=i - 2].iter().sum::<i32>()) * 2;
    }
    // println!("{memo:?}");
    println!("{}", memo[n / 2]);
}
