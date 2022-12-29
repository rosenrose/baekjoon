fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let mut counts = Vec::new();

    for count_3kg in 0..=n / 3 {
        for count_5kg in 0..=(n - count_3kg * 3) / 5 {
            if (count_3kg * 3) + (count_5kg * 5) != n {
                continue;
            }

            counts.push(count_3kg + count_5kg);
        }
    }

    println!("{}", counts.iter().min().unwrap_or(&-1));
}
