fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let radix = buf.trim().chars().count() as i32;
    let n: i32 = buf.trim().parse().unwrap();

    let min = n - (9 * radix);
    let min = if min <= 0 { 1 } else { min };
    let max = n - 1;

    for i in min..=max {
        if d(i) == n {
            println!("{i}");
            return;
        }
    }

    println!("0");
}

fn d(n: i32) -> i32 {
    n + n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .sum::<i32>()
}
