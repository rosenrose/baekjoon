use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let count = buf
        .lines()
        .skip(1)
        .filter(|input| {
            input
                .split('-')
                .next_back()
                .unwrap()
                .parse::<i32>()
                .unwrap()
                <= 90
        })
        .count();

    println!("{count}");
}
