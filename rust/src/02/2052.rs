fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    const EXP: i128 = 10_000_000_000_000_000_000_000_000_000_000_000_000;

    let mut bigint = vec![1 as i128];

    for _ in 0..n {
        let mut carry = 0;

        for num in bigint.iter_mut() {
            *num = carry + *num * 5;

            carry = *num / EXP;
            *num %= EXP;
        }

        if carry > 0 {
            bigint.push(carry);
        }
    }

    let bigint: String = bigint
        .iter()
        .rev()
        .enumerate()
        .map(|(i, num)| {
            if i == 0 {
                num.to_string()
            } else {
                format!("{num:037}")
            }
        })
        .collect();

    println!("0.{}{bigint}", "0".repeat(n - bigint.len()));
}
