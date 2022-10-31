fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let document = buf.trim().to_string();
    read_line(&mut buf);

    let keyword = buf.trim().to_string();

    println!("{}", document.matches(&keyword).count());
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
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
