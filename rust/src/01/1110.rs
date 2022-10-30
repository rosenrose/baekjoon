fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let n = if n < 10 { n * 10 } else { n };

    let mut next = n;
    let mut count = 0;

    loop {
        let left_digit = next % 10;
        let right_digit = ((next / 10) + (next % 10)) % 10;

        next = left_digit * 10 + right_digit;
        count += 1;

        if next == n {
            break;
        }
    }

    println!("{count}");
}
