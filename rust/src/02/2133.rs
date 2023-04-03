fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    if n % 2 == 1 {
        println!("0");
        return;
    }

    let mut memo = vec![0; n / 2 + 1];
    (memo[0], memo[1]) = (1, 3);

    for i in 2..=n / 2 {
        memo[i] = memo[i - 1] * 3 + (memo[0..=i - 2].iter().sum::<i32>()) * 2;
    }
    // println!("{memo:?}");
    println!("{}", memo[n / 2]);
}
