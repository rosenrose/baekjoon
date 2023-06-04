use std::collections::VecDeque;

const MAX_DIGITS: usize = 7;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let [n, k] = parse_int_vec(&buf)[..] else { return };
    const MAX: usize = 1_000_000;

    let mut max_num = 0;
    let mut visited = vec![0; MAX + 1];
    let mut queue = VecDeque::from([(n, 0)]);

    while let Some((num, count)) = queue.pop_front() {
        if count == k {
            max_num = num.max(max_num);
            continue;
        }

        let next_count = count + 1;
        let digits = to_digits(num);
        let first_non_zero = digits.iter().position(|&digit| digit != 0).unwrap();

        for i in first_non_zero..MAX_DIGITS - 1 {
            for j in i + 1..MAX_DIGITS {
                if i == first_non_zero && digits[j] == 0 {
                    continue;
                }

                let mut swapped_digits = digits;
                swapped_digits.swap(i, j);

                let adj = to_int(swapped_digits);

                if visited[adj as usize] == next_count {
                    continue;
                }

                visited[adj as usize] = next_count;
                queue.push_back((adj, next_count));
            }
        }
    }

    if max_num == 0 {
        println!("-1");
        return;
    }

    println!("{max_num}");
}

fn to_digits(mut num: u32) -> [u8; MAX_DIGITS] {
    let mut digits = [0; MAX_DIGITS];

    for digit in digits.iter_mut().rev() {
        *digit = (num % 10) as u8;
        num /= 10;
    }

    digits
}

fn to_int(digits: [u8; MAX_DIGITS]) -> u32 {
    digits.iter().fold(0, |acc, &digit| acc * 10 + digit as u32)
}

fn parse_int_vec(buf: &str) -> Vec<u32> {
    buf.split_whitespace().flat_map(str::parse).collect()
}
