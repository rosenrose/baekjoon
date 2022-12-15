fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    let (mut a, mut count_1) = (1, 1);
    let count_2 = n - 2;

    for _ in 2..n {
        (a, count_1) = (count_1, a + count_1);
    }

    println!("{count_1} {count_2}");
}
