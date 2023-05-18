fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let input = buf.trim().as_bytes();
    let sum: i32 = input
        .iter()
        .map(|ch| {
            (if matches!(ch, b'a'..=b'z') {
                ch - b'a' + 1
            } else {
                ch - b'A' + 27
            }) as i32
        })
        .sum();

    println!(
        "It is {}",
        if is_prime(sum) {
            "a prime word."
        } else {
            "not a prime word."
        }
    );
}

fn is_prime(num: i32) -> bool {
    if num <= 1 {
        return true;
    }

    (2..).take_while(|i| i * i <= num).all(|i| num % i != 0)
}
