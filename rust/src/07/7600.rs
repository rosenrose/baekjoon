use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().take_while(|&input| input != "#") {
        let mut chars = [false; 26];

        for ch in input.to_lowercase().chars() {
            if ch.is_alphabetic() {
                chars[(ch as u8 - b'a') as usize] = true;
            }
        }

        println!("{}", chars.iter().filter(|&&b| b).count());
    }
}
