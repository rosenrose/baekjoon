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

    loop {
        if (1..len).all(|i| word[i - 1] != word[i]) {
            count += 1;
        }

        let Some(i) = (1..len).rfind(|&i| word[i - 1] < word[i]) else {
            break;
        };
        let j = (i..len).rfind(|&j| word[j] > word[i - 1]).unwrap();

        word.swap(i - 1, j);
        (&mut word[i..]).sort();
    }

    println!("{count}");
}
