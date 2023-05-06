fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let times: i32 = buf
        .trim()
        .chars()
        .map(|ch| {
            2 + match ch {
                'A'..='C' => 1,
                'D'..='F' => 2,
                'G'..='I' => 3,
                'J'..='L' => 4,
                'M'..='O' => 5,
                'P'..='S' => 6,
                'T'..='V' => 7,
                'W'..='Z' => 8,
                _ => unreachable!(),
            }
        })
        .sum();

    println!("{times}");
}
