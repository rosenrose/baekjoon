fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut nums = buf.trim().split(':').flat_map(str::parse::<i32>);

    let (a, b) = (nums.next().unwrap(), nums.next().unwrap());
    let gcd = get_gcd(a, b);

    println!("{}:{}", a / gcd, b / gcd);
}

fn get_gcd(mut a: i32, mut b: i32) -> i32 {
    loop {
        if b == 0 {
            return a;
        }

        (a, b) = (b, a % b);
    }
}
