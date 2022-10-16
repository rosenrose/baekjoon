use std::io::{stdin, stdout, BufRead, BufWriter, Write};

fn main() {
    let stdin = stdin();
    let stdout = stdout();
    let mut stdin = stdin.lock();
    let mut stdout = BufWriter::new(stdout.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    let mut members: Vec<(i32, String, i32)> = (0..n)
        .map(|order| {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            let mut tokens = buf.split_whitespace();

            let age: i32 = tokens.next().unwrap().parse().unwrap();
            let name = tokens.next().unwrap();

            (age, name.to_string(), order)
        })
        .collect();

    members.sort_by(|(age1, _, order1), (age2, _, order2)| {
        if age1 == age2 {
            order1.cmp(order2)
        } else {
            age1.cmp(age2)
        }
    });

    for (age, name, _) in members {
        writeln!(stdout, "{age} {name}").unwrap();
    }
}
