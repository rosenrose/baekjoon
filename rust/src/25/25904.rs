use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let (n, x) = (input.next().unwrap(), input.next().unwrap());
    let mut order: Vec<_> = input
        .enumerate()
        .map(|(i, t)| {
            let mut shout = x + i as i32;

            while shout <= t {
                shout += n;
            }

            (shout, i + 1)
        })
        .collect();

    order.sort();

    println!("{}", order[0].1);
}
