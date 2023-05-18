use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut shortcuts = [false; 26];

    'outer: for input in buf.lines().skip(1) {
        let lower = input.to_lowercase();
        let mut i = 0;

        for word in lower.split(' ').map(str::as_bytes) {
            let idx = (word[0] - b'a') as usize;

            if !shortcuts[idx] {
                shortcuts[idx] = true;

                println!("{}[{}]{}", &input[..i], &input[i..i + 1], &input[i + 1..]);

                continue 'outer;
            }

            i += word.len() + 1;
        }

        for (i, &ch) in lower.as_bytes().iter().enumerate() {
            if ch == b' ' {
                continue;
            }

            let idx = (ch - b'a') as usize;

            if !shortcuts[idx] {
                shortcuts[idx] = true;

                println!("{}[{}]{}", &input[..i], &input[i..i + 1], &input[i + 1..]);

                continue 'outer;
            }
        }

        println!("{input}");
    }
}
