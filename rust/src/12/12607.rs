use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    const MAPPING: [&str; 26] = [
        "2", "2", "2", "3", "3", "3", "4", "4", "4", "5", "5", "5", "6", "6", "6", "7", "7", "7",
        "7", "8", "8", "8", "9", "9", "9", "9",
    ];
    let get_key = |ch: u8| {
        if ch == b' ' {
            "0"
        } else {
            MAPPING[(ch - b'a') as usize]
        }
    };

    for (i, input) in buf.lines().map(str::as_bytes).enumerate().skip(1) {
        let mut keypress = String::new();

        for (i, &ch) in input.iter().enumerate() {
            let key = get_key(ch);

            keypress.extend(
                key.repeat(if ch == b' ' {
                    1
                } else {
                    (ch - match ch as char {
                        'a'..='c' => b'a',
                        'd'..='f' => b'd',
                        'g'..='i' => b'g',
                        'j'..='l' => b'j',
                        'm'..='o' => b'm',
                        'p'..='s' => b'p',
                        't'..='v' => b't',
                        'w'..='z' => b'w',
                        _ => unreachable!(),
                    } + 1) as usize
                })
                .chars(),
            );

            let Some(&next) = input.get(i + 1) else {
                continue;
            };
            let next_key = get_key(next);

            if key == next_key {
                keypress.push(' ');
            }
        }

        println!("Case #{i}: {keypress}");
    }
}
