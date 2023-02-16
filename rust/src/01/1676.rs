fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let (mut count_2, mut count_5) = (0, 0);

    let mut pow = 2;
    while pow <= n {
        count_2 += n / pow;
        pow *= 2;
    }

    pow = 5;
    while pow <= n {
        count_5 += n / pow;
        pow *= 5;
    }

    println!("{}", count_2.min(count_5));
}
