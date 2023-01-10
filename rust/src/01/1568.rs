fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut n: i32 = buf.trim().parse().unwrap();
    let mut seconds = 0;
    let sum = |num: i32| (num * (num + 1)) / 2;

    while n > 0 {
        let mut step = 0;
        let step = loop {
            if sum(step) > n {
                break step - 1;
            }

            step += 1;
        };

        seconds += step;
        n -= sum(step);
    }

    println!("{seconds}");
}
