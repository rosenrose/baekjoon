fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut n: i32 = buf.trim().parse().unwrap();

    for end_num in 666.. {
        let mut six_count = 0;
        let mut num = end_num;

        while num > 0 {
            if num % 10 == 6 {
                six_count += 1;
            } else {
                six_count = 0;
            }

            if six_count == 3 {
                n -= 1;
                break;
            }

            num /= 10;
        }

        if n == 0 {
            println!("{end_num}");
            return;
        }
    }
}
