fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let m = parse_int(&buf);
    read_line(&mut buf);
    let n = parse_int(&buf);

    let prime_nums: Vec<i32> = (m..=n).filter(|&n| is_prime(n)).collect();

    if prime_nums.is_empty() {
        println!("{}", -1);
        return;
    }

    println!(
        "{}\n{}",
        prime_nums.iter().sum::<i32>(),
        prime_nums.iter().min().unwrap()
    );
}

fn is_prime(num: i32) -> bool {
    if num == 1 {
        return false;
    }

    for i in (2..).take_while(|i| i * i <= num) {
        if num % i == 0 {
            return false;
        }
    }

    true
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
