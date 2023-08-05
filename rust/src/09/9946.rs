use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut output = String::new();

    let get_counts = |s: &str| {
        let mut counts = [0; 26];

        for ch in s.as_bytes() {
            counts[(ch - b'a') as usize] += 1;
        }

        counts
    };

    for (i, [a, b]) in (1..).map(|i| (i, [(); 2].map(|_| input.next().unwrap()))) {
        if (a, b) == ("END", "END") {
            break;
        }

        writeln!(
            output,
            "Case {i}: {}",
            if get_counts(a) == get_counts(b) {
                "same"
            } else {
                "different"
            }
        )
        .unwrap();
    }

    print!("{output}");
}
