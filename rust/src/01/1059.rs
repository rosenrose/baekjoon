use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let mut s: Vec<_> = (0..input()).map(|_| input()).collect();
    s.sort();

    let n = input();

    if s.contains(&n) {
        println!("0");
        return;
    }

    let (p, &up) = s.iter().enumerate().find(|(_, &num)| num > n).unwrap();
    let down = if p > 0 { s[p - 1] } else { 0 };
    let mut count = 0;

    for len in 1..up - down {
        for x in n - len..=n {
            if x <= down || x + len >= up {
                continue;
            }

            count += 1;
        }
    }

    println!("{count}");
}
