fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    const M: i32 = 1_000_000_000;

    match n {
        1 => println!("9"),
        2 => println!("17"),
        _ => {
            let mut digit_counts = [1, 1, 2, 2, 2, 2, 2, 2, 2, 1];

            for _ in 0..n - 2 {
                digit_counts = std::array::from_fn(|i| match i {
                    0 => digit_counts[1],
                    9 => digit_counts[8],
                    _ => (digit_counts[i - 1] + digit_counts[i + 1]) % M,
                });
            }
            // println!("{digit_counts:?}");
            let stair_nums = digit_counts.into_iter().reduce(|a, b| (a + b) % M).unwrap();

            println!("{stair_nums}");
        }
    }
}
