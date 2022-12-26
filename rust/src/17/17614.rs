fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let mut count = 0;

    for mut num in 1..=n {
        while num > 0 {
            match num % 10 {
                3 | 6 | 9 => count += 1,
                _ => (),
            }

            num /= 10;
        }
    }

    println!("{count}");
}
