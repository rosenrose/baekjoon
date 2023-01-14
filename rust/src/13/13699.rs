fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    let mut t = vec![1];

    for i in 1..=n {
        let next: usize = (0..=i - 1).map(|j| t[j] * t[i - 1 - j]).sum();
        t.push(next);
    }

    println!("{}", t[n]);
}
