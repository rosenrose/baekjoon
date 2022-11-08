fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);

        if buf.trim() == "EOI" {
            return;
        }

        println!(
            "{}",
            if buf.trim().to_lowercase().contains("nemo") {
                "Found"
            } else {
                "Missing"
            }
        );
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
