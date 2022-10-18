fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let letters = buf.trim().chars();
    const DEFAULT_TIME: i32 = 2;

    let times = letters.map(|letter| {
        let mut time = DEFAULT_TIME;

        match letter {
            'A'..='C' => time += 1,
            'D'..='F' => time += 2,
            'G'..='I' => time += 3,
            'J'..='L' => time += 4,
            'M'..='O' => time += 5,
            'P'..='S' => time += 6,
            'T'..='V' => time += 7,
            'W'..='Z' => time += 8,
            _ => (),
        }

        time
    });

    println!("{}", times.sum::<i32>());
}
