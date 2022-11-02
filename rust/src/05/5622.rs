fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let letters = buf.trim().chars();
    const DEFAULT_TIME: i32 = 2;

    let times = letters.map(|letter| {
        DEFAULT_TIME
            + match letter {
                'A'..='C' => 1,
                'D'..='F' => 2,
                'G'..='I' => 3,
                'J'..='L' => 4,
                'M'..='O' => 5,
                'P'..='S' => 6,
                'T'..='V' => 7,
                'W'..='Z' => 8,
                _ => 0,
            }
    });

    println!("{}", times.sum::<i32>());
}
