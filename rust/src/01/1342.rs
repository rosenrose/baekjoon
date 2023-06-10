fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut word = buf.trim().as_bytes().to_vec();
    word.sort();

    let len = word.len();
    let mut count = 0;

    if (1..len).all(|i| word[i - 1] != word[i]) {
        println!("{}", (1..=len).product::<usize>());
        return;
    }

    while next_permutation(&mut word) {
        if (1..len).all(|i| word[i - 1] != word[i]) {
            count += 1;
        }
    }

    println!("{count}");
}

fn next_permutation(chars: &mut Vec<u8>) -> bool {
    let len = chars.len();

    let Some(i) = (1..len).rfind(|&i| chars[i - 1] < chars[i]) else {
        return false;
    };
    let j = (i..len).rfind(|&j| chars[j] > chars[i - 1]).unwrap();

    chars.swap(i - 1, j);
    chars[i..].sort();

    true
}
