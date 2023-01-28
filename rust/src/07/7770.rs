fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i64 = buf.trim().parse().unwrap();
    let min_blocks = |height: i64| (height * (2 * height * height + 1)) / 3;
    let mut step = 0;

    while min_blocks(step) <= n {
        step += 1;
    }

    println!("{}", step - 1);
}
