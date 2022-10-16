fn main() {
    let mut buf = String::new();
    read_line(&mut buf);
    read_line(&mut buf);

    let prime_nums = buf
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .filter(|&n| is_prime(n));

    println!("{}", prime_nums.count());
}

fn is_prime(num: i32) -> bool {
    if num == 1 {
        return false;
    }
    if num == 2 {
        return true;
    }

    for n in 2..=num - 1 {
        if num % n == 0 {
            return false;
        }
    }

    true
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
