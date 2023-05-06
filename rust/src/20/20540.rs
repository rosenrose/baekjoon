fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mbti: String = buf
        .trim()
        .chars()
        .map(|ch| match ch {
            'E' => 'I',
            'I' => 'E',
            'S' => 'N',
            'N' => 'S',
            'T' => 'F',
            'F' => 'T',
            'J' => 'P',
            'P' => 'J',
            _ => unreachable!(),
        })
        .collect();

    println!("{mbti}");
}
