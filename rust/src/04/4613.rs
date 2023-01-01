use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for input in buf.lines().take_while(|&input| input != "#") {
        let checksum: usize = input
            .char_indices()
            .map(|(i, c)| {
                if matches!(c, 'A'..='Z') {
                    (i + 1) * (c as u8 - 'A' as u8 + 1) as usize
                } else {
                    0
                }
            })
            .sum();

        println!("{checksum}");
    }
}
