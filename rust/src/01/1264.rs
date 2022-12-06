use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

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
