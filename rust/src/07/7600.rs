use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().take_while(|&input| input != "#") {
        let letters = input
            .to_lowercase()
            .chars()
            .fold([false; 26], |mut acc, ch| {
                if ch.is_alphabetic() {
                    acc[ch as usize - 'a' as usize] = true;
                }

                acc
            });

        println!("{}", letters.iter().filter(|&&b| b).count());
    }
}
