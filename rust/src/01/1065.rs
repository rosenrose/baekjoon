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

    let digits: Vec<_> = num.to_string().chars().map(|c| c as i8).collect();
    let diff = digits[0] - digits[1];

    (2..digits.len()).all(|i| digits[i - 1] - digits[i] == diff)
}
