use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().map(str::as_bytes);

    let (plain, key) = (input.next().unwrap(), input.next().unwrap());
    let offset = 'a' as u8;

    let encrypted: String = plain
        .chunks(key.len())
        .flat_map(|chunk| {
            chunk.iter().enumerate().map(|(i, &ch)| {
                if ch as char == ' ' {
                    return ' ';
                }

                (((ch - offset) + (26 - (key[i] - offset + 1))) % 26 + offset) as char
            })
        })
        .collect();

    println!("{encrypted}");
}
