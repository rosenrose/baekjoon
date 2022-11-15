fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let x = buf.trim();
    let mut count = 0;

    let sum = if x.len() < 2 {
        x.parse().unwrap()
    } else {
        convert(x.to_string(), &mut count)
    };

    println!("{count}\n{}", if sum % 3 == 0 { "YES" } else { "NO" });
}

fn convert(num: String, count: &mut i32) -> u32 {
    *count += 1;

    let sum: u32 = num.chars().map(|c| c.to_digit(10).unwrap()).sum();

    if sum < 10 {
        sum
    } else {
        convert(sum.to_string(), count)
    }
}