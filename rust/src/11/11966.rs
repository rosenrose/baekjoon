fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    for bit in (0..).take_while(|&b| (1 << b) < n) {
        if ((1 << bit) & n) >> bit == 1 {
            println!("0");
            return;
        }
    }

    println!("1");
}
