use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    for input in buf.lines().take_while(|&input| input != "#") {
        let vowels = input
            .to_lowercase()
            .chars()
            .filter(|c| match c {
                'a' | 'e' | 'i' | 'o' | 'u' => true,
                _ => false,
            })
            .count();

        println!("{vowels}");
    }
}
