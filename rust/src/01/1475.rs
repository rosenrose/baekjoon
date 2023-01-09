fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut counts = buf.trim().chars().fold([0; 10], |mut acc, ch| {
        acc[ch as usize - '0' as usize] += 1;
        acc
    });

    let count_69 = counts[6] + counts[9];
    let count_69 = (count_69 + 1) / 2;

    (counts[6], counts[9]) = (count_69, count_69);

    println!("{}", counts.iter().max().unwrap());
}
