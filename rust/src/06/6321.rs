use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let offset = b'A';

    for (i, name) in buf.lines().map(str::as_bytes).enumerate().skip(1) {
        let new_name: String = name
            .iter()
            .map(|ch| ((ch - offset + 1) % 26 + offset) as char)
            .collect();

        println!("String #{i}");
        println!("{new_name}\n");
    }
}
