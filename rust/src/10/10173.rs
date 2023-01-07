use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    for input in buf.lines().take_while(|&input| input != "EOI") {
        println!(
            "{}",
            if input.to_lowercase().contains("nemo") {
                "Found"
            } else {
                "Missing"
            }
        );
    }
}
