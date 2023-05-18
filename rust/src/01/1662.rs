fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    println!("{}", unzipped_len(buf.trim()));
}

fn unzipped_len(input: &str) -> i32 {
    let mut len = 0;
    let mut substr = String::new();
    let mut repeat_times = 0;
    let mut count = 0;

    for ch in input.chars() {
        if count > 0 {
            substr.push(ch);
        }

        match ch {
            '(' => {
                if count == 0 {
                    len -= 1;
                }

                count += 1;
            }
            ')' => {
                count -= 1;

                if count == 0 {
                    len += repeat_times * unzipped_len(&substr);
                    substr.clear();
                }
            }
            _ => {
                if count == 0 {
                    len += 1;
                    repeat_times = (ch as u8 - b'0') as i32;
                }
            }
        }
    }

    len
}
