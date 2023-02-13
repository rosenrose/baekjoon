use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    const MAPPING: [&str; 26] = [
        "2", "2", "2", "3", "3", "3", "4", "4", "4", "5", "5", "5", "6", "6", "6", "7", "7", "7",
        "7", "8", "8", "8", "9", "9", "9", "9",
    ];
    let get_key = |ch: char| {
        if ch == ' ' {
            "0"
        } else {
            MAPPING[ch as usize - 'a' as usize]
        }
    };

    for (i, input) in buf.lines().map(str::as_bytes).enumerate().skip(1) {
        let mut keypress = String::new();

        for idx in 0..input.len() {
            let ch = input[idx] as char;
            let key = get_key(ch);

            keypress.extend(
                key.repeat(if ch == ' ' {
                    1
                } else {
                    ch as usize
                        - match ch {
                            'a'..='c' => 'a' as usize,
                            'd'..='f' => 'd' as usize,
                            'g'..='i' => 'g' as usize,
                            'j'..='l' => 'j' as usize,
                            'm'..='o' => 'm' as usize,
                            'p'..='s' => 'p' as usize,
                            't'..='v' => 't' as usize,
                            'w'..='z' => 'w' as usize,
                            _ => Default::default(),
                        }
                        + 1
                })
                .chars(),
            );

            let Some(&next) = input.get(idx + 1) else {
                continue;
            };
            let next_key = get_key(next as char);

            if key == next_key {
                keypress.push(' ');
            }
        }

        println!("Case #{i}: {keypress}");
    }
}
