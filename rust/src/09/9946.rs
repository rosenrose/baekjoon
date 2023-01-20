use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut output = String::new();

    let char_count = |s: &str| {
        let mut count = [0; 26];

        for ch in s.chars() {
            count[ch as usize - 'a' as usize] += 1;
        }

        count
    };

    for (i, (a, b)) in (1..).map(|i| (i, (input.next().unwrap(), input.next().unwrap()))) {
        if (a, b) == ("END", "END") {
            break;
        }

        writeln!(
            output,
            "Case {i}: {}",
            if char_count(a) == char_count(b) {
                "same"
            } else {
                "different"
            }
        )
        .unwrap();
    }

    print!("{output}");
}
