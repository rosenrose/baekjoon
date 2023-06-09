use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    const BUG: &str = "BUG";

    for mut input in buf.lines().map(str::to_owned) {
        while input.contains(BUG) {
            input = input.replace(BUG, "");
        }

        println!("{input}");
    }
}
