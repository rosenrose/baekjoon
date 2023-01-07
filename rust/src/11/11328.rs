use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let n: i32 = input.next().unwrap().parse().unwrap();

    for _ in 0..n {
        let (a, b) = (input.next().unwrap(), input.next().unwrap());

        if a.len() != b.len() {
            writeln!(output, "Impossible").unwrap();
            continue;
        }

        let (mut a_counts, mut b_counts) = ([0; 26], [0; 26]);

        for ch in a.chars() {
            a_counts[ch as usize - 'a' as usize] += 1;
        }

        for ch in b.chars() {
            let index = ch as usize - 'a' as usize;

            if a_counts[index] == 0 {
                break;
            }

            b_counts[index] += 1;
        }

        writeln!(
            output,
            "{}",
            if a_counts == b_counts {
                "Possible"
            } else {
                "Impossible"
            }
        )
        .unwrap();
    }

    print!("{output}");
}
