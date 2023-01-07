use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
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
