use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let n: i32 = input().parse().unwrap();
    let char_count = |s: &str| {
        s.chars().fold([0; 26], |mut acc, ch| {
            acc[ch as usize - 'a' as usize] += 1;
            acc
        })
    };

    for (a, b) in (0..n).map(|_| (input(), input())) {
        println!(
            "{a} & {b} are {}",
            if char_count(a) == char_count(b) {
                "anagrams."
            } else {
                "NOT anagrams."
            }
        );
    }
}
