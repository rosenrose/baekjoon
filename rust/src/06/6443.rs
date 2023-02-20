use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for input in buf.lines().skip(1) {
        let mut word = input.as_bytes().to_vec();
        word.sort();

        let len = word.len();

        loop {
            writeln!(output, "{}", String::from_utf8(word.clone()).unwrap()).unwrap();

            let Some(i) = (1..len).rfind(|&i| word[i - 1] < word[i]) else {
                break;
            };
            let j = (i..len).rfind(|&j| word[j] > word[i - 1]).unwrap();

            word.swap(i - 1, j);
            (&mut word[i..]).sort();
        }
    }

    print!("{output}");
}
