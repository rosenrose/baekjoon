use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
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
