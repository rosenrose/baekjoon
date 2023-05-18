use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for input in buf.lines().take_while(|&input| input != "#") {
        let checksum: usize = input
            .as_bytes()
            .iter()
            .enumerate()
            .map(|(i, ch)| {
                if matches!(ch, b'A'..=b'Z') {
                    (i + 1) * (ch - b'A' + 1) as usize
                } else {
                    0
                }
            })
            .sum();

        println!("{checksum}");
    }
}
