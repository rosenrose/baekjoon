fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let (a, b) = get_ab(&buf.trim()).unwrap();

    println!("{a} {b}");
}

fn get_ab(s: &str) -> Option<(usize, usize)> {
    let len = s.len();

    if len <= 2 {
        let num = s.parse().unwrap();

        return match len {
            1 => Some((num, num)),
            2 => {
                let arr: Vec<usize> = s
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect();
                let (first, second) = (arr[0], arr[1]);

                if second as i32 - first as i32 == 1 {
                    Some((first, second))
                } else {
                    Some((num, num))
                }
            }
            _ => None,
        };
    }

    'a_loop: for a_size in 1..=3.min(len) {
        let a: usize = s[..a_size].parse().unwrap();

        for i in 1..=(4 - a_size) {
            let after_a_size = (a + i).to_string().chars().count();
            let after_a_start = (a..a + i).map(|n| n.to_string().chars().count()).sum();

            if after_a_start + after_a_size > len {
                continue;
            }
            // println!("a: {a} {after_a_start} {}", after_a_start + after_a_size);
            let after_a: usize = s[after_a_start..after_a_start + after_a_size]
                .parse()
                .unwrap();

            if after_a != a + i {
                continue 'a_loop;
            }
        }

        if a_size == len {
            return Some((a, a));
        }

        'b_loop: for b_size in a_size..=3.min(len - a_size) {
            let b_start = len - b_size;
            let b: usize = s[b_start..len].parse().unwrap();

            if b < a {
                continue;
            }

            for i in 1..=(4 - b_size) {
                if i > b {
                    continue 'b_loop;
                }

                let before_b_size = (b - i).to_string().chars().count();
                let before_b_start: usize =
                    (b - i + 1..=b).map(|n| n.to_string().chars().count()).sum();
                let before_b_start = len as i32 - before_b_start as i32 - before_b_size as i32;

                if before_b_start < 0 {
                    break;
                }
                // println!(
                //     "b: {b} {before_b_start} {}",
                //     before_b_start as usize + before_b_size
                // );

                let before_b: usize = s
                    [before_b_start as usize..before_b_start as usize + before_b_size]
                    .parse()
                    .unwrap();

                if before_b != b - i {
                    continue 'b_loop;
                }
            }

            return Some((a, b));
        }
    }

    None
}
