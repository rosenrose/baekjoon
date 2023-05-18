use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    const FAKE: &str = "FAKE";

    'outer: for input in buf.lines().skip(1) {
        let chars = input.as_bytes();
        let mut counts = [0; 26];

        for (i, ch) in chars.iter().enumerate() {
            let idx = (ch - b'A') as usize;
            counts[idx] += 1;

            if counts[idx] == 3 {
                let Some(next) = chars.get(i + 1) else {
                    println!("{FAKE}");
                    continue 'outer;
                };

                if ch != next {
                    println!("{FAKE}");
                    continue 'outer;
                }
            }

            if counts[idx] == 4 {
                counts[idx] = 0;
            }
        }

        println!("OK");
    }
}
