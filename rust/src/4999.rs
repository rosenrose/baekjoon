fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let ah = buf.trim().to_string();
    read_line(&mut buf);

    let require = buf.trim().to_string();

    println!(
        "{}",
        if ah.len() >= require.len() {
            "go"
        } else {
            "no"
        }
    );
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
