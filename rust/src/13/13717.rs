use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let (mut sum, mut max_count) = (0, 0);
    let counts: Vec<_> = (0..parse_int(input()))
        .map(|_| {
            let (name, k, mut m) = (input(), parse_int(input()), parse_int(input()));
            let mut count = 0;

            while m >= k {
                m -= k - 2;
                count += 1;
            }

            sum += count;
            max_count = count.max(max_count);

            (name, count)
        })
        .collect();

    let (max_name, _) = counts.iter().find(|(_, c)| *c == max_count).unwrap();

    println!("{sum}\n{max_name}");
}

fn parse_int(buf: &str) -> i32 {
    buf.parse().unwrap()
}
