use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let regex = Regex::new("^da+dd?(i|y)$", false);

    for input in buf.lines() {
        println!(
            "{}",
            if regex.find(input).is_some() {
                "She called me!!!"
            } else {
                "Cooing"
            }
        );
    }
}
