fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let (mut count_2, mut count_5) = (0, 0);

    (1..=n).for_each(|i| {
        let mut num = i;

        while num % 2 == 0 {
            num /= 2;
            count_2 += 1;
        }

        num = i;

        while num % 5 == 0 {
            num /= 5;
            count_5 += 1;
        }
    });

    println!("{}", count_2.min(count_5));
}
