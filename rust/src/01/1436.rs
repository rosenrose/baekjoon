fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    let mut num = 666;
    let mut count = 0;

    loop {
        let mut six_count = 0;

        for digit in num.to_string().chars() {
            if digit == '6' {
                six_count += 1;
            } else {
                six_count = 0;
            }

            if six_count == 3 {
                count += 1;
                break;
            }
        }

        if count == n {
            break;
        }

        num += 1;
    }

    println!("{num}");
}
