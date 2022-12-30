use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    println!(
        "{}",
        if buf.lines().any(|input| input == "anj") {
            "뭐야;"
        } else {
            "뭐야?"
        }
    );
}
