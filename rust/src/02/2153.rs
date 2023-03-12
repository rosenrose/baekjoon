fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let sum: i32 = buf
        .trim()
        .chars()
        .map(|ch| {
            if matches!(ch, 'a'..='z') {
                ch as i32 - 'a' as i32 + 1
            } else {
                ch as i32 - 'A' as i32 + 27
            }
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
