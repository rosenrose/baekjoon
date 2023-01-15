use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    while let (Some(n @ 4..), Some(p @ 1..)) = (input.next(), input.next()) {
        let lost_pages = match (p <= n / 2, p % 2) {
            (true, 1) => [p, p + 1, n - p, n + 1 - p],
            (true, 0) => [p - 1, p, n + 1 - p, n + 2 - p],
            (false, 1) => [n - p, n + 1 - p, p, p + 1],
            (false, 0) => [n + 1 - p, n + 2 - p, p - 1, p],
            _ => Default::default(),
        };

        for page in lost_pages.iter().filter(|&&page| page != p) {
            print!("{page} ");
        }
        println!("");
    }
}
