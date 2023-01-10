use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().take_while(|&input| input != "#") {
        let dec: i32 = input
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| {
                (match c {
                    '-' => 0,
                    '\\' => 1,
                    '(' => 2,
                    '@' => 3,
                    '?' => 4,
                    '>' => 5,
                    '&' => 6,
                    '%' => 7,
                    _ => -1,
                }) * 8_i32.pow(i as u32)
            })
            .sum();

        println!("{dec}");
    }
}
