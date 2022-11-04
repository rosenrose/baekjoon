fn main() {
    let mut buf = String::new();
    let mut is_found = false;

    for i in 1..=5 {
        read_line(&mut buf);

        if buf.contains("FBI") {
            is_found = true;
            print!("{i} ");
        }
    }

    if !is_found {
        println!("HE GOT AWAY!");
    }
}

fn read_line(buf: &mut String) {
    buf.clear();
    std::io::stdin().read_line(buf).unwrap();
}
