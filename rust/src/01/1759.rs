use core::char;
use std::io;

const MAX: usize = 15;
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let [len, chars_len] = [(); 2].map(|_| input.next().unwrap().parse::<usize>().unwrap());
    let mut chars = ['\0'; MAX];

    for (i, ch) in input.flat_map(|s| s.chars().nth(0)).enumerate() {
        chars[i] = ch;
    }

    chars[..chars_len].sort();

    combinations(0, 0, &mut ['\0'; MAX][..len], &chars[..chars_len]);
}

fn combinations(depth: usize, start: usize, selected: &mut [char], chars: &[char]) {
    if depth == selected.len() {
        let result = String::from_iter(selected.iter());

        if result.matches(VOWELS).count() < 1 {
            return;
        }
        if result.matches(|ch| !VOWELS.contains(&ch)).count() < 2 {
            return;
        }

        println!("{result}");

        return;
    }

    let takes = chars.len() - (selected.len() - 1);

    for i in start..depth + takes {
        selected[depth] = chars[i];
        combinations(depth + 1, i + 1, selected, chars);
    }
}
