fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let letters = buf.trim().chars();
    const DEFAULT_TIME: i32 = 2;

    let times = letters.map(|letter| {
        let mut time = DEFAULT_TIME;

        match letter {
            l if ('A'..='C').contains(&l) => time += 1,
            l if ('D'..='F').contains(&l) => time += 2,
            l if ('G'..='I').contains(&l) => time += 3,
            l if ('J'..='L').contains(&l) => time += 4,
            l if ('M'..='O').contains(&l) => time += 5,
            l if ('P'..='S').contains(&l) => time += 6,
            l if ('T'..='V').contains(&l) => time += 7,
            l if ('W'..='Z').contains(&l) => time += 8,
            _ => (),
        }

        time
    });

    println!("{}", times.sum::<i32>());
}
