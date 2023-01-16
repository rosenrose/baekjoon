use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().take_while(|&input| input != "#") {
        let vowels = input
            .to_lowercase()
            .matches(['a', 'e', 'i', 'o', 'u'])
            .count();

        println!("{vowels}");
    }
}
