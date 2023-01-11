use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    for mut input in buf.lines().map(|line| line.to_string()) {
        while input.contains("BUG") {
            input = input.replace("BUG", "");
        }

        println!("{input}");
    }
}
