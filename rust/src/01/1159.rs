use std::io;

const MAX: usize = 26;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut counts = [0; MAX];
    let offset = b'a';

    for ch in buf.lines().skip(1).map(|input| input.as_bytes()[0]) {
        counts[(ch - offset) as usize] += 1;
    }

    let mut availables = ['\0'; MAX];
    let mut availables_len = 0;

    for ch in counts
        .iter()
        .enumerate()
        .filter_map(|(ch, &count)| (count >= 5).then_some((ch as u8 + offset) as char))
    {
        availables[availables_len] = ch;
        availables_len += 1;
    }

    if availables_len == 0 {
        println!("PREDAJA");
        return;
    }

    availables[..availables_len].sort();

    println!("{}", String::from_iter(&availables[..availables_len]));
}
