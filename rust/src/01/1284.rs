use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.split_ascii_whitespace();

    for num in input.take_while(|&num| num != "0") {
        let width = num
            .chars()
            .map(|c| {
                (match c {
                    '1' => 2,
                    '0' => 4,
                    _ => 3,
                }) + 1
            })
            .sum::<i32>()
            + 1;

        println!("{width}");
    }
}
