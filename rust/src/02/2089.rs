fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut n: i32 = buf.trim().parse().unwrap();
    let mut digits = Vec::new();

    while n != 0 {
        digits.push(n.rem_euclid(-2));
        n = n.div_euclid(-2);
    }

    if digits.is_empty() {
        println!("0");
        return;
    }

    for d in digits.iter().rev() {
        print!("{d}");
    }
}
