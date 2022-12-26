use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let input = buf.lines();

    for (i, name) in input.enumerate().skip(1) {
        let offset = 'A' as u8;
        let new_name: String = name
            .chars()
            .map(|c| ((c as u8 - offset + 1) % 26 + offset) as char)
            .collect();

        println!("String #{i}");
        println!("{new_name}\n");
    }
}
