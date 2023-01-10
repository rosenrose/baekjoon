use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut output = String::new();

    (1..)
        .map(|i| (i, (input.next().unwrap(), input.next().unwrap())))
        .take_while(|&(_, (a, b))| (a, b) != ("END", "END"))
        .for_each(|(i, (a, b))| {
            let (mut a_count, mut b_count) = ([0; 26], [0; 26]);

            for ch in a.chars() {
                a_count[ch as usize - 'a' as usize] += 1;
            }
            for ch in b.chars() {
                b_count[ch as usize - 'a' as usize] += 1;
            }

            writeln!(
                output,
                "Case {i}: {}",
                if a_count == b_count {
                    "same"
                } else {
                    "different"
                }
            )
            .unwrap();
        });

    print!("{output}");
}
