use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    buf.lines().for_each(|line| {
        let mut line = line.to_string();

        while line.contains("BUG") {
            line = line.replace("BUG", "");
        }

        println!("{line}");
    });
}
