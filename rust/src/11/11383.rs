use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let n: i32 = input.next().unwrap().parse().unwrap();
    input.next();

    let first: Vec<String> = (0..n)
        .map(|_| {
            input
                .next()
                .unwrap()
                .chars()
                .map(|c| c.to_string().repeat(2))
                .collect()
        })
        .collect();

    println!(
        "{}",
        if input.zip(first).all(|(second, first)| first == second) {
            "Eyfa"
        } else {
            "Not Eyfa"
        }
    );
}
