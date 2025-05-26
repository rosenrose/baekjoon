use std::io;

const MAX: usize = 35 + 1;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut t = [0; MAX];
    t[0] = 1;

    for i in 1..=n {
        let next: usize = (0..=i - 1).map(|j| t[j] * t[i - 1 - j]).sum();
        t[i] = next;
    }

    println!("{}", t[n]);
}
