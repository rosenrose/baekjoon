fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    println!(
        "{}",
        buf.trim()
            .chars()
            .map(|c| match c {
                'E' => 'I',
                'I' => 'E',
                'S' => 'N',
                'N' => 'S',
                'T' => 'F',
                'F' => 'T',
                'J' => 'P',
                'P' => 'J',
                _ => Default::default(),
            })
            .collect::<String>()
    );
}
