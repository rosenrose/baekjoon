fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let mut dots_per_side = 2;

    for _ in 0..n {
        dots_per_side = dots_per_side * 2 - 1;
    }

    println!("{}", dots_per_side * dots_per_side);
}
