fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let s = buf.trim();
    let s_len = s.len();

    if s_len <= 2 {
        let num: i32 = buf.trim().parse().unwrap();

        match s_len {
            1 => println!("{num} {num}"),
            2 => {
                let mut digits = s.chars().map(|c| c as u8 - '0' as u8);
                let (a, b) = (digits.next().unwrap(), digits.next().unwrap());

                if b - a == 1 {
                    println!("{a} {b}");
                } else {
                    println!("{num} {num}")
                }
            }
            _ => unreachable!(),
        }

        return;
    }

    let (mut a, mut b) = (0, 0);
    let mut index;

    'a_size: for a_len in 1..=3 {
        a = s[0..a_len].parse().unwrap();
        index = a_len;

        if a_len == 3 {
            break;
        }

        for i in 1..=4 - a_len {
            let after_a = a + i;
            let after_a_len = after_a.ilog10() as usize + 1;

            let check: usize = s[index..(index + after_a_len).min(s_len)].parse().unwrap();

            if check != after_a {
                continue 'a_size;
            }

            index += after_a_len;

            if index >= s_len {
                break;
            }
        }

        break;
    }

    let a_len = a.ilog10() as usize + 1;

    if a_len == s_len {
        println!("{a} {a}");
        return;
    }

    'b_size: for b_len in a_len..=3.min(s_len - a_len) {
        index = s_len - b_len;
        b = s[index..index + b_len].parse().unwrap();

        if b < a {
            continue;
        }
        if b_len == 3 {
            break;
        }

        for i in 1..=4 - b_len {
            let before_b = b - i;
            let before_b_len = before_b.ilog10() as usize + 1;

            let check: usize = s[index.saturating_sub(before_b_len)..index]
                .parse()
                .unwrap();

            if check != before_b {
                continue 'b_size;
            }

            if index as i32 - (before_b_len as i32) <= 0 {
                break;
            }

            index -= before_b_len;
        }

        break;
    }

    println!("{a} {b}");
}
