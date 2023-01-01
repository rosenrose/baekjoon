fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let mut count = 0;

    for mut num in 1..=n {
        let number = num;
        let mut sum = 0;

        while num > 0 {
            sum += num % 10;
            num /= 10;
        }

        if number % sum == 0 {
            count += 1;
        }
    }

    println!("{count}");
}
