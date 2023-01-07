use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
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
