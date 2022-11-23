fn main() {
    let mut buf = String::new();

    loop {
        read_line(&mut buf);

        if buf.trim() == "0" {
            return;
        }

        let width = buf
            .trim()
            .chars()
            .map(|c| {
                (match c {
                    '1' => 2,
                    '0' => 4,
                    _ => 3,
                }) + 1
            })
            .sum::<i32>()
            + 1;

        println!("{width}");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
