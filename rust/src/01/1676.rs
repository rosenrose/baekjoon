fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let (mut count_2, mut count_5) = (0, 0);

    (2..=n).step_by(2).for_each(|mut i| {
        while i % 2 == 0 {
            i /= 2;
            count_2 += 1;
        }
    });

    (5..=n).step_by(5).for_each(|mut i| {
        while i % 5 == 0 {
            i /= 5;
            count_5 += 1;
        }
    });

    println!("{}", count_2.min(count_5));
}
