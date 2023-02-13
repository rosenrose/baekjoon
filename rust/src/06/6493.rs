use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let regex = Regex::new("^(AB)*A(H|((D|E)F+)+GC)C*((D|E)F+)+G$", false);

    for input in buf.lines().skip(1) {
        let ab_count = input.matches("AB").count();
        let mut c_count = input.matches('C').count();

        if !input.contains('H') {
            c_count -= 1;
        }

        println!(
            "{}",
            if regex.find(input).is_some() && ab_count == c_count {
                "YES"
            } else {
                "NO"
            }
        );
    }
}
