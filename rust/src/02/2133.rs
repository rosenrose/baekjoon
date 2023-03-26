fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    if n % 2 == 1 {
        println!("0");
        return;
    }

    let mut cache = vec![0; n / 2 + 1];
    cache[1] = 3;

    for i in 2..=n / 2 {
        cache[i] = cache[i - 1] * 3 + (cache[1..=i - 2].iter().sum::<i32>() + 1) * 2;
    }
    // println!("{cache:?}");
    println!("{}", cache[n / 2]);
}
