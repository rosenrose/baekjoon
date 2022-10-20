fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let m = parse_int(&buf);
    read_line(&mut buf);
    let n = parse_int(&buf);

    let prime_nums: Vec<i32> = (m..=n).filter(|&n| is_prime(n)).collect();

    if prime_nums.len() == 0 {
        println!("{}", -1);
        return;
    }

    let sum: i32 = prime_nums.iter().sum();
    let min = prime_nums.iter().min().unwrap();

    println!("{sum}\n{min}");
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
