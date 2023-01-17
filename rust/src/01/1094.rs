fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut n: u8 = buf.trim().parse().unwrap();
    let mut count = 0;

    while n > 0 {
        if n & 1 == 1 {
            count += 1;
        }

        n >>= 1;
    }

    println!("{count}");
}
