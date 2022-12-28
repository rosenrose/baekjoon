fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut n: i32 = buf.trim().parse().unwrap();

    while n > 2 {
        if n & 1 == 1 {
            println!("0");
            return;
        }

        n >>= 1;
    }

    println!("1");
}
