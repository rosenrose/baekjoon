fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let digits = buf.trim().chars().map(|c| c.to_digit(10).unwrap() as usize);
    let mut counts = [0; 10];

    for num in digits {
        counts[num] += 1;
    }

    let count_69 = counts[6] + counts[9];
    let count_69 = if count_69 % 2 == 0 {
        count_69 / 2
    } else {
        (count_69 + 1) / 2
    };

    counts[6] = count_69;
    counts[9] = count_69;

    println!("{}", counts.iter().max().unwrap());
}
