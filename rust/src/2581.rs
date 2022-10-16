fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let m = parse_int(&buf);
    read_line(&mut buf);

    let n = parse_int(&buf);

    let prime_nums = (m..=n).filter(|&n| is_prime(n));

    if prime_nums.clone().count() == 0 {
        println!("{}", -1);
        return;
    }

    println!("{}", prime_nums.clone().sum::<i32>());
    println!("{}", prime_nums.min().unwrap());
}

fn is_prime(num: i32) -> bool {
    if num == 1 {
        return false;
    }
    if num == 2 {
        return true;
    }

    for n in 2..=(num as f64).sqrt() as i32 {
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

fn parse_int(buf: &String) -> i32 {
    buf.trim().parse().unwrap()
}
