use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut input = || input.next().unwrap();

    let n: i32 = input().parse().unwrap();

    for (encrypted, mapping) in (0..n).map(|_| (input(), input().as_bytes())) {
        let decrypted = String::from_iter(encrypted.chars().map(|ch| {
            if ch.is_alphabetic() {
                mapping[ch as usize - 'A' as usize] as char
            } else {
                ch
            }
        }));

        println!("{decrypted}");
    }
}
