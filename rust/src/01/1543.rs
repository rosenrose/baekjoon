fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let document = buf.trim().to_string();
    read_line(&mut buf);

    let keyword = buf.trim().to_string();
    let (doc_len, key_len) = (document.len(), keyword.len());

    if doc_len < key_len {
        println!("0");
        return;
    }

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

    println!("{count}");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
