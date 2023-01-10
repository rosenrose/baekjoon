use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().take_while(|&input| input != "#") {
        let parity = input.chars().last().unwrap();
        let count_1 = input.matches('1').count();

        let last = match (parity, count_1 % 2) {
            ('e', 0) | ('o', 1) => '0',
            _ => '1',
        };

        println!("{}{last}", &input[..input.len() - 1]);
    }
}
