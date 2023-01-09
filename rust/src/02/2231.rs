fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let input = buf.trim();
    let n: i32 = input.parse().unwrap();

    let min = (n - (9 * input.len() as i32)).max(1);
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
    let digit_sum: i32 = n.to_string().chars().map(|c| c as i32 - '0' as i32).sum();
    n + digit_sum
}
