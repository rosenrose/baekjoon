use std::collections::VecDeque;
use std::io;

const NUM_MAX: usize = 1_000_000;
const DIGIT_MAX: usize = 7;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<u32>);

    let [n, k] = [(); 2].map(|_| input.next().unwrap());

    let mut max_num = 0;
    let mut visited = [0; NUM_MAX + 1];
    let mut queue = VecDeque::from([(n, 0)]);

    while let Some((num, count)) = queue.pop_front() {
        if count == k {
            max_num = num.max(max_num);
            continue;
        }

        let next_count = count + 1;
        let digits = to_digits(num);
        let first_non_zero = digits.iter().position(|&digit| digit != 0).unwrap();

        for i in first_non_zero..DIGIT_MAX - 1 {
            for j in i + 1..DIGIT_MAX {
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

fn to_digits(mut num: u32) -> [u8; DIGIT_MAX] {
    let mut digits = [0; DIGIT_MAX];

    for digit in digits.iter_mut().rev() {
        *digit = (num % 10) as u8;
        num /= 10;
    }

    digits
}

fn to_int(digits: [u8; DIGIT_MAX]) -> u32 {
    digits.iter().fold(0, |acc, &digit| acc * 10 + digit as u32)
}
