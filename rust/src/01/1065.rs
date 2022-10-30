fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let count = (1..=n).filter(|&i| is_hansu(i)).count();

    println!("{count}");
}

fn is_hansu(num: i32) -> bool {
    if num < 100 {
        return true;
    }

    let digits: Vec<i32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let diff = digits[0] - digits[1];

    for i in 1..digits.len() - 1 {
        if diff != digits[i] - digits[i + 1] {
            return false;
        }
    }

    true
}
