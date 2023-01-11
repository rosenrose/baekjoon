use std::io;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    let len: i32 = input.next().unwrap().parse().unwrap();
    input.next();

    let mut chars: Vec<_> = input.collect();
    chars.sort();

    combination(&chars, len, 0, &mut String::new());
}

fn combination(chars: &Vec<&str>, m: i32, start: usize, selected: &mut String) {
    if m == 0 {
        if selected.matches(VOWELS).count() < 1 {
            return;
        }
        if selected.matches(|c| !VOWELS.contains(&c)).count() < 2 {
            return;
        }

        println!("{selected}");

        return;
    }

    for i in start..chars.len() - (m as usize - 1) {
        if selected.contains(chars[i]) {
            continue;
        }

        selected.push_str(chars[i]);

        combination(chars, m - 1, i + 1, selected);

        selected.pop();
    }
}
