use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().take_while(|&input| input != "#") {
        let ch = input.chars().nth(0).unwrap();

        println!("{ch} {}", input.to_lowercase().matches(ch).count() - 1);
    }
}
