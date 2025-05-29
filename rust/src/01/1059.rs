use std::io;

const MAX: usize = 50;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let size = input.next().unwrap() as usize;
    let mut s = [0; MAX];

    for (i, num) in input.by_ref().take(size).enumerate() {
        s[i] = num;
    }

    s[..size].sort();
    let n = input.next().unwrap();

    if s[..size].contains(&n) {
        println!("0");
        return;
    }

    let (p, &up) = s[..size]
        .iter()
        .enumerate()
        .find(|(_, &num)| num > n)
        .unwrap();
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
