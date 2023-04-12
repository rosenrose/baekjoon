use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().take_while(|&input| input != "#") {
        let mut letters = [false; 26];

        for ch in input.to_lowercase().chars() {
            if ch.is_alphabetic() {
                letters[ch as usize - 'a' as usize] = true;
            }
        }

        println!("{}", letters.iter().filter(|&&b| b).count());
    }
}
