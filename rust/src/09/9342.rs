use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let regex = Regex::new("^[A-F]?A+F+C+[A-F]?$", false);

    for input in buf.lines().skip(1) {
        println!(
            "{}",
            if regex.find(input).is_some() {
                "Infected!"
            } else {
                "Good"
            }
        );
    }
}
