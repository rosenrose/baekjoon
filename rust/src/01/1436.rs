fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    let mut num = 666;
    let mut count = 0;

    loop {
        let mut six_count = 0;
        let mut i = num;

        while i > 0 {
            if i % 10 == 6 {
                six_count += 1;
            } else {
                six_count = 0;
            }

            if six_count == 3 {
                count += 1;
                break;
            }

            i /= 10;
        }

        if count == n {
            break;
        }

        num += 1;
    }

    println!("{num}");
}
