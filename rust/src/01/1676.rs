fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let (mut count_2, mut count_5) = (0, 0);

    let mut exp = 2;
    while exp <= n {
        count_2 += n / exp;
        exp *= 2;
    }

    exp = 5;
    while exp <= n {
        count_5 += n / exp;
        exp *= 5;
    }

    println!("{}", count_2.min(count_5));
}
