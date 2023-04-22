fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();
    let (mut start, mut end) = (1, 1);
    let mut sum = 1;
    let mut count = 0;

    while end <= n {
        if sum < n {
            end += 1;
            sum += end;
            continue;
        }

        if sum == n {
            count += 1;
        }

        sum -= start;
        start += 1;
    }

    println!("{count}");
}
