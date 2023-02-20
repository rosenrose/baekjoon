use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for word in buf.lines().skip(1) {
        if let Some(next) = next_permutation(word.as_bytes().to_vec()) {
            println!("{}", String::from_utf8(next).unwrap());
        } else {
            println!("BIGGEST");
        }
    }
}

fn next_permutation(mut chars: Vec<u8>) -> Option<Vec<u8>> {
    let len = chars.len();

    let i = (1..len).rfind(|&i| chars[i - 1] < chars[i])?;
    let j = (i..len).rfind(|&j| chars[j] > chars[i - 1]).unwrap();

    chars.swap(i - 1, j);
    (&mut chars[i..]).sort();

    Some(chars)
}
