use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    for mut num in input.take_while(|&n| n != 0) {
        while num >= 10 {
            num = num
                .to_string()
                .as_bytes()
                .iter()
                .map(|ch| (ch - b'0') as i32)
                .sum();
        }

        println!("{num}");
    }
}
