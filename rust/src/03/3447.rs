use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    buf.lines().for_each(|line| {
        let mut line = line.to_string();

        while line.contains("BUG") {
            line = line.replace("BUG", "");
        }

        println!("{line}");
    });
}
