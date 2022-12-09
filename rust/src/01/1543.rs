use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.lines();
    let (document, keyword) = (input.next().unwrap(), input.next().unwrap());

    println!("{}", document.matches(keyword).count());
}

/*
let mut count = 0;
let mut i = 0;

while i + key_len <= doc_len {
    if &document[i..i + key_len] == &keyword[..] {
        count += 1;
        i += key_len;

        continue;
    }

    i += 1;
}
 */
