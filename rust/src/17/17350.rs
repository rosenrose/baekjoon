use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    println!(
        "{}",
        if buf.lines().any(|input| input == "anj") {
            "뭐야;"
        } else {
            "뭐야?"
        }
    );
}
