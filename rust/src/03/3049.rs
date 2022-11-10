fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let points = (n - 3..=n).product::<i32>() / (1..=4).product::<i32>();

    println!("{points}");
}
