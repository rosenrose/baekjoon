use std::io;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let len: usize = input.next().unwrap().parse().unwrap();
    let mut chars: Vec<_> = input.skip(1).flat_map(|s| s.chars().nth(0)).collect();
    chars.sort();

    combinations(0, 0, &mut vec!['\0'; len], &chars);
}

fn combinations(depth: usize, start: usize, selected: &mut Vec<char>, chars: &Vec<char>) {
    if depth == selected.len() {
        let result = String::from_iter(selected.iter());

        if result.matches(VOWELS).count() < 1 {
            return;
        }
        if result.matches(|c| !VOWELS.contains(&c)).count() < 2 {
            return;
        }

        println!("{result}");

        return;
    }

    let takes = chars.len() - selected.len() + 1;

    for (i, &ch) in chars.iter().enumerate().skip(start).take(takes) {
        if selected[..depth].contains(&ch) {
            continue;
        }

        selected[depth] = ch;
        combinations(depth + 1, i + 1, selected, chars);
    }
}
