fn main() {
    let mut buf = String::new();
    read_line(&mut buf);

    let n: i32 = buf.trim().parse().unwrap();
    read_line(&mut buf);

    let before = buf.trim().to_string();
    read_line(&mut buf);

    let after = buf.trim().to_string();

    for (x, y) in before.chars().zip(after.chars()) {
        match (n % 2, x == y) {
            (0, false) | (1, true) => {
                println!("Deletion failed");
                return;
            }
            _ => (),
        };
    }

    println!("Deletion succeeded");
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
